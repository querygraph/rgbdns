---
type: "code-file"
source_path: "src/multilog.rs"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
line_count: 314
fragment_count: 28
rgbdns_commit: "79502939"
---

# src/multilog.rs

- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]
- Source path: `src/multilog.rs`
- Lines: 314
- Summary: Bounded daemontools-compatible log collection and rotation.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-3805c0fd2354|DEFAULT_MAX_SIZE]]: lines 10-10
- [[DNS from First Principles/Fragments/rgbdns-frag-1f75b4128b39|DEFAULT_RETAIN]]: lines 11-11
- [[DNS from First Principles/Fragments/rgbdns-frag-dc8d3b346dce|MAX_DESTINATIONS]]: lines 12-12
- [[DNS from First Principles/Fragments/rgbdns-frag-4b799706056f|MAX_SIZE]]: lines 13-13
- [[DNS from First Principles/Fragments/rgbdns-frag-d18b7d8ac600|MAX_RETAIN]]: lines 14-16
- [[DNS from First Principles/Fragments/rgbdns-frag-3e7a8fc52fc3|Config]]: lines 17-23
- [[DNS from First Principles/Fragments/rgbdns-frag-25e572c5969f|Config]]: lines 24-24
- [[DNS from First Principles/Fragments/rgbdns-frag-be0b815b7b09|parse]]: lines 25-61
- [[DNS from First Principles/Fragments/rgbdns-frag-8d1f39317994|parse_bounded]]: lines 62-72
- [[DNS from First Principles/Fragments/rgbdns-frag-995359454c38|run]]: lines 73-108
- [[DNS from First Principles/Fragments/rgbdns-frag-677455483b37|Log]]: lines 109-117
- [[DNS from First Principles/Fragments/rgbdns-frag-3a91a9d8a5e2|Log]]: lines 118-118
- [[DNS from First Principles/Fragments/rgbdns-frag-847df0d6c56e|open]]: lines 119-133
- [[DNS from First Principles/Fragments/rgbdns-frag-5ebafeb3a185|write]]: lines 134-137
- [[DNS from First Principles/Fragments/rgbdns-frag-a2e3aa103cac|write_parts]]: lines 138-148
- [[DNS from First Principles/Fragments/rgbdns-frag-60e772541619|flush]]: lines 149-152
- [[DNS from First Principles/Fragments/rgbdns-frag-0cdd60d75e10|rotate]]: lines 153-173
- [[DNS from First Principles/Fragments/rgbdns-frag-dc66c6b94bff|prune]]: lines 174-194
- [[DNS from First Principles/Fragments/rgbdns-frag-bed5d669e0e8|secure_append]]: lines 195-205
- [[DNS from First Principles/Fragments/rgbdns-frag-d62a2400fa8c|tai64n]]: lines 206-209
- [[DNS from First Principles/Fragments/rgbdns-frag-9230ff72f2c9|tai64n_label]]: lines 210-214
- [[DNS from First Principles/Fragments/rgbdns-frag-ab93be956cc5|tests]]: lines 215-218
- [[DNS from First Principles/Fragments/rgbdns-frag-28cd1b8dc5d1|directory]]: lines 219-223
- [[DNS from First Principles/Fragments/rgbdns-frag-e0c3c03feb8d|parses_supported_daemontools_options]]: lines 224-234
- [[DNS from First Principles/Fragments/rgbdns-frag-346d019845d7|timestamps_streamed_lines_without_buffering_the_input]]: lines 235-253
- [[DNS from First Principles/Fragments/rgbdns-frag-323d4d71bfa4|rotation_does_not_separate_a_timestamp_from_its_line]]: lines 254-270
- [[DNS from First Principles/Fragments/rgbdns-frag-bd83643c4555|rotates_atomically_and_enforces_retention]]: lines 271-295
- [[DNS from First Principles/Fragments/rgbdns-frag-6bf6af6b6066|refuses_a_symlinked_current_file]]: lines 296-314

## Full Source

```rust
//! Bounded daemontools-compatible log collection and rotation.

use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
    time::SystemTime,
};

const DEFAULT_MAX_SIZE: u64 = 99_999;
const DEFAULT_RETAIN: usize = 10;
const MAX_DESTINATIONS: usize = 64;
const MAX_SIZE: u64 = 1 << 40;
const MAX_RETAIN: usize = 10_000;

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
    format!("{} ", crate::tai64::label(time))
}

fn tai64n_label(time: SystemTime) -> String {
    crate::tai64::label(time)
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
```
