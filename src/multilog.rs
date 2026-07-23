//! Bounded daemontools-compatible log collection and rotation.

use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

const DEFAULT_MAX_SIZE: u64 = 99_999;
const DEFAULT_RETAIN: usize = 10;
const MAX_DESTINATIONS: usize = 64;
const MAX_SIZE: u64 = 1 << 40;
const MAX_RETAIN: usize = 10_000;
const TAI64_BASE: u64 = 0x4000_0000_0000_000a;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub timestamp: bool,
    pub max_size: u64,
    pub retain: usize,
    pub directories: Vec<PathBuf>,
}

impl Config {
    pub fn parse(arguments: &[String]) -> Result<Self, String> {
        let mut timestamp = false;
        let mut max_size = DEFAULT_MAX_SIZE;
        let mut retain = DEFAULT_RETAIN;
        let mut directories = Vec::new();
        for argument in arguments {
            if argument == "t" {
                timestamp = true;
            } else if let Some(value) = argument.strip_prefix('s') {
                max_size = parse_bounded(value, 1, MAX_SIZE, "log size")?;
            } else if let Some(value) = argument.strip_prefix('n') {
                retain = parse_bounded(value, 1, MAX_RETAIN, "retention count")?;
            } else if argument.starts_with('-')
                || argument.starts_with('+')
                || argument.starts_with('e')
                || argument.starts_with('E')
            {
                return Err(format!("unsupported multilog selector: {argument}"));
            } else {
                directories.push(PathBuf::from(argument));
            }
        }
        if directories.is_empty() {
            return Err("multilog requires at least one log directory".into());
        }
        if directories.len() > MAX_DESTINATIONS {
            return Err("too many multilog destinations".into());
        }
        Ok(Self {
            timestamp,
            max_size,
            retain,
            directories,
        })
    }
}

fn parse_bounded<T>(value: &str, minimum: T, maximum: T, label: &str) -> Result<T, String>
where
    T: Copy + Ord + std::str::FromStr,
{
    let value = value.parse::<T>().map_err(|_| format!("invalid {label}"))?;
    if !(minimum..=maximum).contains(&value) {
        return Err(format!("{label} is outside the supported range"));
    }
    Ok(value)
}

pub fn run<R: BufRead>(config: &Config, mut input: R) -> io::Result<()> {
    let mut logs = config
        .directories
        .iter()
        .map(|directory| Log::open(directory, config.max_size, config.retain))
        .collect::<io::Result<Vec<_>>>()?;
    let mut line_start = true;
    loop {
        let available = input.fill_buf()?;
        if available.is_empty() {
            break;
        }
        let length = available
            .iter()
            .position(|byte| *byte == b'\n')
            .map_or(available.len(), |position| position + 1);
        let segment = &available[..length];
        if line_start && config.timestamp {
            let prefix = tai64n(SystemTime::now());
            for log in &mut logs {
                log.write_parts(prefix.as_bytes(), segment)?;
            }
        } else {
            for log in &mut logs {
                log.write(segment)?;
            }
        }
        line_start = segment.last() == Some(&b'\n');
        input.consume(length);
    }
    for log in &mut logs {
        log.flush()?;
    }
    Ok(())
}

struct Log {
    directory: PathBuf,
    file: File,
    size: u64,
    maximum: u64,
    retain: usize,
    sequence: u32,
}

impl Log {
    fn open(directory: &Path, maximum: u64, retain: usize) -> io::Result<Self> {
        fs::create_dir_all(directory)?;
        let path = directory.join("current");
        let file = secure_append(&path)?;
        let size = file.metadata()?.len();
        Ok(Self {
            directory: directory.to_owned(),
            file,
            size,
            maximum,
            retain,
            sequence: 0,
        })
    }

    fn write(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.write_parts(bytes, &[])
    }

    fn write_parts(&mut self, first: &[u8], second: &[u8]) -> io::Result<()> {
        let added = (first.len() as u64).saturating_add(second.len() as u64);
        if self.size > 0 && self.size.saturating_add(added) > self.maximum {
            self.rotate()?;
        }
        self.file.write_all(first)?;
        self.file.write_all(second)?;
        self.size = self.size.saturating_add(added);
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }

    fn rotate(&mut self) -> io::Result<()> {
        self.file.flush()?;
        self.file.sync_all()?;
        let current = self.directory.join("current");
        let mut rotated;
        loop {
            let stamp = tai64n_label(SystemTime::now());
            rotated = self
                .directory
                .join(format!("{stamp}.{:08x}.s", self.sequence));
            self.sequence = self.sequence.wrapping_add(1);
            if !rotated.exists() {
                break;
            }
        }
        fs::rename(&current, rotated)?;
        self.file = secure_append(&current)?;
        self.size = 0;
        self.prune()
    }

    fn prune(&self) -> io::Result<()> {
        let mut rotated = fs::read_dir(&self.directory)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                entry.file_type().is_ok_and(|kind| kind.is_file())
                    && name.starts_with('@')
                    && name.ends_with(".s")
            })
            .map(|entry| entry.path())
            .collect::<Vec<_>>();
        rotated.sort();
        let remove = rotated.len().saturating_sub(self.retain);
        for path in rotated.into_iter().take(remove) {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}

fn secure_append(path: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.create(true).append(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o644).custom_flags(nix::libc::O_NOFOLLOW);
    }
    options.open(path)
}

pub fn tai64n(time: SystemTime) -> String {
    format!("{} ", tai64n_label(time))
}

fn tai64n_label(time: SystemTime) -> String {
    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    format!(
        "@{:016x}{:08x}",
        TAI64_BASE.saturating_add(duration.as_secs()),
        duration.subsec_nanos()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn directory(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!("rgbdns-multilog-{label}-{}", std::process::id()))
    }

    #[test]
    fn parses_supported_daemontools_options() {
        let config = Config::parse(&["t", "s1024", "n5", "./main"].map(str::to_owned)).unwrap();
        assert!(config.timestamp);
        assert_eq!(config.max_size, 1024);
        assert_eq!(config.retain, 5);
        assert_eq!(config.directories, [PathBuf::from("./main")]);
        assert!(Config::parse(&["s0".into(), "main".into()]).is_err());
        assert!(Config::parse(&["t".into()]).is_err());
    }

    #[test]
    fn timestamps_streamed_lines_without_buffering_the_input() {
        let path = directory("timestamp");
        let config = Config {
            timestamp: true,
            max_size: 10_000,
            retain: 2,
            directories: vec![path.clone()],
        };
        run(&config, Cursor::new(b"one\ntwo\n")).unwrap();
        let contents = fs::read_to_string(path.join("current")).unwrap();
        let lines = contents.lines().collect::<Vec<_>>();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].starts_with("@4"));
        assert!(lines[0].ends_with(" one"));
        assert!(lines[1].ends_with(" two"));
        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn rotation_does_not_separate_a_timestamp_from_its_line() {
        let path = directory("record-boundary");
        let config = Config {
            timestamp: true,
            max_size: 40,
            retain: 2,
            directories: vec![path.clone()],
        };
        run(&config, Cursor::new(b"alpha\nbeta\n")).unwrap();
        let current = fs::read_to_string(path.join("current")).unwrap();
        assert!(current.starts_with("@4"));
        assert!(current.ends_with(" beta\n"));
        assert!(!current.starts_with("beta"));
        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn rotates_atomically_and_enforces_retention() {
        let path = directory("rotate");
        let config = Config {
            timestamp: false,
            max_size: 4,
            retain: 2,
            directories: vec![path.clone()],
        };
        run(&config, Cursor::new(b"aaaa\nbbbb\ncccc\n")).unwrap();
        let rotated = fs::read_dir(&path)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let name = entry.file_name();
                assert!(!name.to_string_lossy().contains(' '));
                name != "current"
            })
            .count();
        assert_eq!(rotated, 2);
        assert_eq!(fs::read(path.join("current")).unwrap(), b"cccc\n");
        fs::remove_dir_all(path).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn refuses_a_symlinked_current_file() {
        use std::os::unix::fs::symlink;

        let path = directory("symlink");
        fs::create_dir(&path).unwrap();
        let target = path.join("target");
        fs::write(&target, b"unchanged").unwrap();
        symlink(&target, path.join("current")).unwrap();
        let config = Config {
            timestamp: false,
            max_size: 100,
            retain: 2,
            directories: vec![path.clone()],
        };
        assert!(run(&config, Cursor::new(b"attack\n")).is_err());
        assert_eq!(fs::read(target).unwrap(), b"unchanged");
        fs::remove_dir_all(path).unwrap();
    }
}
