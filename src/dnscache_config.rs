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
}
