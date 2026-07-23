//! Atomic implementation of the original `tinydns-edit add` operations.

use crate::{Error, Name, Result};
use std::{
    fs::{self, File},
    io::Write,
    net::Ipv4Addr,
    path::Path,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    Ns,
    ChildNs,
    Host,
    Alias,
    Mx,
}

impl Mode {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "ns" => Ok(Self::Ns),
            "childns" => Ok(Self::ChildNs),
            "host" => Ok(Self::Host),
            "alias" => Ok(Self::Alias),
            "mx" => Ok(Self::Mx),
            _ => Err(Error::Format("invalid tinydns-edit mode")),
        }
    }
}

pub fn add(
    data: &Path,
    temporary: &Path,
    mode: Mode,
    target: Name,
    address: Ipv4Addr,
) -> Result<()> {
    if data == temporary {
        return Err(Error::Format("data and temporary paths must differ"));
    }
    let contents = fs::read_to_string(data)?;
    let mut used = [false; 26];
    let mut ttl = match mode {
        Mode::Ns | Mode::ChildNs => 259_200,
        _ => 86_400,
    };
    for raw in contents.lines() {
        let line = raw.trim_end_matches([' ', '\t', '\r']);
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let marker = line.as_bytes()[0];
        let fields = split_fields(&line[1..]);
        match mode {
            Mode::Ns | Mode::ChildNs => {
                let wanted = if mode == Mode::Ns { b'.' } else { b'&' };
                if marker == wanted && name_field(&fields, 0).as_ref() == Some(&target) {
                    ttl = number(&fields, 3, 259_200);
                    mark_slot(&mut used, &fields, 2, "ns", &target);
                }
            }
            Mode::Host if marker == b'=' => {
                if name_field(&fields, 0).as_ref() == Some(&target) {
                    return Err(Error::InvalidRecord("host name already used".into()));
                }
                if fields
                    .get(1)
                    .and_then(|value| value.parse::<Ipv4Addr>().ok())
                    == Some(address)
                {
                    return Err(Error::InvalidRecord("IP address already used".into()));
                }
            }
            Mode::Mx if marker == b'@' && name_field(&fields, 0).as_ref() == Some(&target) => {
                ttl = number(&fields, 4, 86_400);
                mark_slot(&mut used, &fields, 2, "mx", &target);
            }
            _ => {}
        }
    }
    let owner = target.to_string().trim_end_matches('.').to_owned();
    let line = match mode {
        Mode::Ns | Mode::ChildNs | Mode::Mx => {
            let slot = used
                .iter()
                .position(|used| !used)
                .ok_or_else(|| Error::InvalidRecord("too many records for that domain".into()))?;
            let letter = char::from(b'a' + slot as u8);
            match mode {
                Mode::Ns => format!(".{owner}:{address}:{letter}:{ttl}"),
                Mode::ChildNs => format!("&{owner}:{address}:{letter}:{ttl}"),
                Mode::Mx => format!("@{owner}:{address}:{letter}::{ttl}"),
                _ => unreachable!(),
            }
        }
        Mode::Host => format!("={owner}:{address}:{ttl}"),
        Mode::Alias => format!("+{owner}:{address}:{ttl}"),
    };

    let mut file = File::create(temporary)?;
    let result: Result<()> = (|| {
        file.write_all(contents.as_bytes())?;
        if !contents.is_empty() && !contents.ends_with('\n') {
            file.write_all(b"\n")?;
        }
        writeln!(file, "{line}")?;
        file.sync_all()?;
        Ok(())
    })();
    if let Err(error) = result {
        drop(file);
        let _ = fs::remove_file(temporary);
        return Err(error);
    }
    drop(file);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = fs::metadata(data)?.permissions().mode() & 0o644;
        fs::set_permissions(temporary, fs::Permissions::from_mode(mode))?;
    }
    fs::rename(temporary, data)?;
    Ok(())
}

fn mark_slot(used: &mut [bool; 26], fields: &[String], index: usize, role: &str, owner: &Name) {
    let Some(value) = fields.get(index) else {
        return;
    };
    let expanded = if value.contains('.') {
        value.clone()
    } else {
        format!("{value}.{role}.{owner}")
    };
    let Ok(expanded) = expanded.parse::<Name>() else {
        return;
    };
    for (index, slot) in used.iter_mut().enumerate() {
        let candidate = format!("{}.{role}.{owner}", char::from(b'a' + index as u8));
        if candidate.parse::<Name>().ok().as_ref() == Some(&expanded) {
            *slot = true;
            break;
        }
    }
}

fn name_field(fields: &[String], index: usize) -> Option<Name> {
    fields.get(index)?.parse().ok()
}

fn number(fields: &[String], index: usize, default: u32) -> u32 {
    fields
        .get(index)
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

fn split_fields(value: &str) -> Vec<String> {
    let mut fields = vec![String::new()];
    let mut escaped = false;
    for character in value.chars() {
        if character == ':' && !escaped {
            fields.push(String::new());
        } else {
            fields.last_mut().unwrap().push(character);
        }
        escaped = character == '\\' && !escaped;
    }
    fields
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    fn paths() -> (std::path::PathBuf, std::path::PathBuf) {
        let stem = format!(
            "rgbdns-edit-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        (
            std::env::temp_dir().join(&stem),
            std::env::temp_dir().join(format!("{stem}.new")),
        )
    }

    #[test]
    fn allocates_next_ns_and_mx_slots_atomically() {
        let (data, temporary) = paths();
        fs::write(
            &data,
            ".example:192.0.2.1:a:300\n@example:192.0.2.2:a::400\n",
        )
        .unwrap();
        add(
            &data,
            &temporary,
            Mode::Ns,
            "example".parse().unwrap(),
            "192.0.2.3".parse().unwrap(),
        )
        .unwrap();
        add(
            &data,
            &temporary,
            Mode::Mx,
            "example".parse().unwrap(),
            "192.0.2.4".parse().unwrap(),
        )
        .unwrap();
        let result = fs::read_to_string(&data).unwrap();
        fs::remove_file(data).unwrap();
        assert!(result.contains(".example:192.0.2.3:b:300\n"));
        assert!(result.contains("@example:192.0.2.4:b::400\n"));
    }

    #[test]
    fn host_mode_rejects_duplicate_owner_or_address() {
        let (data, temporary) = paths();
        fs::write(&data, "=host.example:192.0.2.1:60\n").unwrap();
        assert!(
            add(
                &data,
                &temporary,
                Mode::Host,
                "host.example".parse().unwrap(),
                "192.0.2.2".parse().unwrap(),
            )
            .is_err()
        );
        assert!(
            add(
                &data,
                &temporary,
                Mode::Host,
                "other.example".parse().unwrap(),
                "192.0.2.1".parse().unwrap(),
            )
            .is_err()
        );
        fs::remove_file(data).unwrap();
    }
}
