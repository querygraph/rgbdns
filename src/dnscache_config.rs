//! Compatibility handling for djbdns `dnscache` root-server files.

use crate::{Error, Result};
use getrandom::fill;
use std::{
    env, fs,
    io::Write,
    net::IpAddr,
    path::{Path, PathBuf},
};

const MAX_ROOTS_FILE: u64 = 1 << 20;
const MAX_ROOT_ADDRESSES: usize = 256;
const MAX_FORWARD_ZONES: usize = 1024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForwardZone {
    pub name: String,
    pub servers: Vec<IpAddr>,
}

/// A root-hints path suitable for the recursive resolver.
///
/// Native djbdns `root/servers/@` files contain one address per line. Hickory
/// consumes a DNS master file, so legacy input is translated into a private
/// temporary master file and removed when this value is dropped.
pub struct PreparedRoots {
    path: PathBuf,
    temporary: bool,
}

impl PreparedRoots {
    pub fn from_environment() -> Result<Self> {
        let path = env::var_os("ROOTS").map_or_else(
            || {
                env::var_os("ROOT")
                    .map(PathBuf::from)
                    .map(|root| root.join("servers/@"))
                    .unwrap_or_else(|| PathBuf::from("config/root.hints"))
            },
            PathBuf::from,
        );
        Self::prepare(path)
    }

    pub fn prepare(path: PathBuf) -> Result<Self> {
        let metadata = fs::metadata(&path)?;
        if metadata.len() > MAX_ROOTS_FILE {
            return Err(Error::Format("root hints file is too large"));
        }
        let contents = fs::read_to_string(&path)?;
        let lines = contents
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .collect::<Vec<_>>();
        if lines.is_empty() {
            return Err(Error::Format("root hints file contains no servers"));
        }

        let addresses = lines
            .iter()
            .map(|line| line.parse::<IpAddr>())
            .collect::<std::result::Result<Vec<_>, _>>();
        let Ok(addresses) = addresses else {
            // Reject mixed legacy/master input, but leave a DNS master file to
            // Hickory's strict parser for full syntax validation.
            if lines.iter().any(|line| line.parse::<IpAddr>().is_ok()) {
                return Err(Error::Format("mixed root hints file formats"));
            }
            return Ok(Self {
                path,
                temporary: false,
            });
        };
        if addresses.len() > MAX_ROOT_ADDRESSES {
            return Err(Error::Format("too many root server addresses"));
        }

        let mut master = String::new();
        for (index, address) in addresses.iter().enumerate() {
            let host = format!("root-{index}.rgbdns.invalid.");
            master.push_str(&format!(". 3600000 NS {host}\n"));
            let rr_type = if address.is_ipv4() { "A" } else { "AAAA" };
            master.push_str(&format!("{host} 3600000 {rr_type} {address}\n"));
        }
        let path = private_temporary_path()?;
        let result = write_private(&path, master.as_bytes());
        if result.is_err() {
            let _ = fs::remove_file(&path);
        }
        result?;
        Ok(Self {
            path,
            temporary: true,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// Loads original djbdns `ROOT/servers/domain` forwarding rules.
pub fn forward_zones_from_environment() -> Result<Vec<ForwardZone>> {
    let Some(root) = env::var_os("ROOT").map(PathBuf::from) else {
        return Ok(Vec::new());
    };
    load_forward_zones(&root.join("servers"))
}

pub fn load_forward_zones(directory: &Path) -> Result<Vec<ForwardZone>> {
    let mut zones = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        if zones.len() >= MAX_FORWARD_ZONES {
            return Err(Error::Format("too many forwarding zones"));
        }
        let file_type = entry.file_type()?;
        if !file_type.is_file() {
            continue;
        }
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| Error::Format("forwarding zone name is not UTF-8"))?;
        if name == "@" {
            continue;
        }
        if name.is_empty() || name.starts_with('.') || name.ends_with('.') {
            return Err(Error::Format("invalid forwarding zone filename"));
        }
        let metadata = entry.metadata()?;
        if metadata.len() > MAX_ROOTS_FILE {
            return Err(Error::Format("forwarding server file is too large"));
        }
        let contents = fs::read_to_string(entry.path())?;
        let servers = contents
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(|line| {
                line.parse::<IpAddr>()
                    .map_err(|_| Error::Format("invalid forwarding server address"))
            })
            .collect::<Result<Vec<_>>>()?;
        if servers.is_empty() {
            return Err(Error::Format("forwarding zone contains no servers"));
        }
        if servers.len() > MAX_ROOT_ADDRESSES {
            return Err(Error::Format("too many forwarding server addresses"));
        }
        zones.push(ForwardZone { name, servers });
    }
    zones.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(zones)
}

impl Drop for PreparedRoots {
    fn drop(&mut self) {
        if self.temporary {
            let _ = fs::remove_file(&self.path);
        }
    }
}

fn private_temporary_path() -> Result<PathBuf> {
    for _ in 0..32 {
        let mut random = [0_u8; 16];
        fill(&mut random)
            .map_err(|_| Error::Io(std::io::Error::other("OS randomness unavailable")))?;
        let suffix = random
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        let path = env::temp_dir().join(format!("rgbdns-roots-{suffix}.zone"));
        if !path.exists() {
            return Ok(path);
        }
    }
    Err(Error::Io(std::io::Error::new(
        std::io::ErrorKind::AlreadyExists,
        "unable to allocate temporary root hints file",
    )))
}

fn write_private(path: &Path, contents: &[u8]) -> Result<()> {
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path)?;
    file.write_all(contents)?;
    file.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn path(label: &str) -> PathBuf {
        env::temp_dir().join(format!("rgbdns-roots-test-{label}-{}", std::process::id()))
    }

    #[test]
    fn converts_legacy_ipv4_and_ipv6_server_lines() {
        let source = path("legacy");
        fs::write(&source, "# roots\n198.41.0.4\n2001:503:ba3e::2:30\n").unwrap();
        let prepared = PreparedRoots::prepare(source.clone()).unwrap();
        assert_ne!(prepared.path(), source);
        let master = fs::read_to_string(prepared.path()).unwrap();
        assert!(master.contains(" A 198.41.0.4"));
        assert!(master.contains(" AAAA 2001:503:ba3e::2:30"));
        let temporary = prepared.path().to_owned();
        drop(prepared);
        assert!(!temporary.exists());
        fs::remove_file(source).unwrap();
    }

    #[test]
    fn preserves_master_files_and_rejects_mixed_input() {
        let master = path("master");
        fs::write(
            &master,
            ". 3600000 NS a.root.\na.root. 3600000 A 192.0.2.1\n",
        )
        .unwrap();
        let prepared = PreparedRoots::prepare(master.clone()).unwrap();
        assert_eq!(prepared.path(), master);
        drop(prepared);
        assert!(master.exists());
        fs::remove_file(master).unwrap();

        let mixed = path("mixed");
        fs::write(&mixed, "192.0.2.1\nnot-an-address\n").unwrap();
        assert!(PreparedRoots::prepare(mixed.clone()).is_err());
        fs::remove_file(mixed).unwrap();
    }

    #[test]
    fn loads_bounded_per_zone_forwarders() {
        let directory = path("forwarders");
        fs::create_dir(&directory).unwrap();
        fs::write(directory.join("@"), "198.41.0.4\n").unwrap();
        fs::write(
            directory.join("internal.example"),
            "192.0.2.53\n2001:db8::53\n",
        )
        .unwrap();
        assert_eq!(
            load_forward_zones(&directory).unwrap(),
            vec![ForwardZone {
                name: "internal.example".into(),
                servers: vec![
                    "192.0.2.53".parse().unwrap(),
                    "2001:db8::53".parse().unwrap()
                ],
            }]
        );
        fs::remove_dir_all(directory).unwrap();
    }
}
