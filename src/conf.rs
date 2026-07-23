//! Portable service-directory generators for the djbdns daemons.

use crate::{Error, Result};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Clone, Copy, Debug)]
pub enum Service {
    Tinydns,
    Dnscache,
    Rbldns,
    Pickdns,
    Walldns,
    Axfrdns,
}

pub fn configure(service: Service, arguments: &[String]) -> Result<()> {
    let (user, log_user, directory, ip, extra) = match service {
        Service::Tinydns | Service::Pickdns | Service::Walldns if arguments.len() == 4 => (
            arguments[0].as_str(),
            arguments[1].as_str(),
            Path::new(&arguments[2]),
            arguments[3].as_str(),
            None,
        ),
        Service::Dnscache if arguments.len() == 3 || arguments.len() == 4 => (
            arguments[0].as_str(),
            arguments[1].as_str(),
            Path::new(&arguments[2]),
            arguments.get(3).map_or("127.0.0.1", String::as_str),
            None,
        ),
        Service::Rbldns if arguments.len() == 5 => (
            arguments[0].as_str(),
            arguments[1].as_str(),
            Path::new(&arguments[2]),
            arguments[3].as_str(),
            Some(arguments[4].as_str()),
        ),
        Service::Axfrdns if arguments.len() == 5 => (
            arguments[0].as_str(),
            arguments[1].as_str(),
            Path::new(&arguments[2]),
            arguments[4].as_str(),
            Some(arguments[3].as_str()),
        ),
        _ => return Err(Error::Format("invalid service configuration arguments")),
    };
    if !directory.is_absolute()
        || extra.is_some_and(|path| {
            matches!(service, Service::Axfrdns) && !Path::new(path).is_absolute()
        })
    {
        return Err(Error::Format("service directories must be absolute"));
    }
    fs::create_dir(directory)?;
    make_log(directory, log_user)?;
    fs::create_dir(directory.join("env"))?;
    write_file(
        &directory.join("env/IP"),
        format!("{ip}\n").as_bytes(),
        0o644,
    )?;

    let root = if matches!(service, Service::Axfrdns) {
        PathBuf::from(extra.unwrap()).join("root")
    } else {
        directory.join("root")
    };
    write_file(
        &directory.join("env/ROOT"),
        format!("{}\n", root.display()).as_bytes(),
        0o644,
    )?;
    if !matches!(service, Service::Axfrdns) {
        fs::create_dir(&root)?;
    }

    match service {
        Service::Tinydns => configure_tinydns(directory, &root)?,
        Service::Dnscache => configure_dnscache(directory, &root)?,
        Service::Rbldns => {
            write_file(
                &directory.join("env/BASE"),
                format!("{}\n", extra.unwrap()).as_bytes(),
                0o644,
            )?;
            write_file(&root.join("data"), b"", 0o644)?;
            write_file(
                &root.join("Makefile"),
                format!(
                    "data.cdb: data\n\t{}\n",
                    executable("rbldns-data")?.display()
                )
                .as_bytes(),
                0o644,
            )?;
        }
        Service::Pickdns => {
            write_file(&root.join("data"), b"", 0o644)?;
            write_file(
                &root.join("Makefile"),
                format!(
                    "data.cdb: data\n\t{}\n",
                    executable("pickdns-data")?.display()
                )
                .as_bytes(),
                0o644,
            )?;
        }
        Service::Walldns | Service::Axfrdns => {}
    }

    if matches!(service, Service::Dnscache) {
        write_file(&directory.join("env/CACHESIZE"), b"1000000\n", 0o644)?;
    }
    if matches!(service, Service::Axfrdns) {
        write_file(
            &directory.join("env/ALLOW_NETS"),
            b"127.0.0.0/8,::1/128\n",
            0o644,
        )?;
    }
    let binary = match service {
        Service::Tinydns => "tinydns",
        Service::Dnscache => "dnscache",
        Service::Rbldns => "rbldns",
        Service::Pickdns => "pickdns",
        Service::Walldns => "walldns",
        Service::Axfrdns => "axfrdns",
    };
    write_file(
        &directory.join("run"),
        run_script(directory, user, binary)?.as_bytes(),
        0o755,
    )?;
    Ok(())
}

fn configure_tinydns(directory: &Path, root: &Path) -> Result<()> {
    write_file(&root.join("data"), b"", 0o644)?;
    write_file(
        &root.join("Makefile"),
        format!(
            "data.cdb: data\n\t{}\n",
            executable("tinydns-data")?.display()
        )
        .as_bytes(),
        0o644,
    )?;
    for (script, mode) in [
        ("add-ns", "ns"),
        ("add-childns", "childns"),
        ("add-host", "host"),
        ("add-alias", "alias"),
        ("add-mx", "mx"),
    ] {
        write_file(
            &root.join(script),
            format!(
                "#!/bin/sh\nexec {} data data.new add {mode} \"$@\"\n",
                shell_quote(&executable("tinydns-edit")?.to_string_lossy())
            )
            .as_bytes(),
            0o755,
        )?;
    }
    let _ = directory;
    Ok(())
}

fn configure_dnscache(directory: &Path, root: &Path) -> Result<()> {
    let hints = include_bytes!("../config/root.hints");
    fs::create_dir(root.join("servers"))?;
    write_file(&root.join("servers/@"), hints, 0o644)?;
    write_file(
        &directory.join("env/ROOTS"),
        format!("{}\n", root.join("servers/@").display()).as_bytes(),
        0o644,
    )?;
    let mut seed = [0; 128];
    getrandom::fill(&mut seed)
        .map_err(|_| Error::Io(std::io::Error::other("OS randomness unavailable")))?;
    write_file(&directory.join("seed"), &seed, 0o600)
}

fn make_log(directory: &Path, user: &str) -> Result<()> {
    fs::create_dir(directory.join("log"))?;
    fs::create_dir(directory.join("log/main"))?;
    write_file(
        &directory.join("log/run"),
        format!(
            "#!/bin/sh\nexec setuidgid {} multilog t ./main\n",
            shell_quote(user)
        )
        .as_bytes(),
        0o755,
    )
}

fn run_script(directory: &Path, user: &str, binary: &str) -> Result<String> {
    Ok(format!(
        "#!/bin/sh\nset -eu\nROOT=$(cat {dir}/env/ROOT)\nIP=$(cat {dir}/env/IP)\nexport ROOT IP DATA=data.cdb\n\
         [ ! -f {dir}/env/BASE ] || export BASE=$(cat {dir}/env/BASE)\n\
         [ ! -f {dir}/env/ROOTS ] || export ROOTS=$(cat {dir}/env/ROOTS)\n\
         [ ! -f {dir}/env/CACHESIZE ] || export CACHESIZE=$(cat {dir}/env/CACHESIZE)\n\
         [ ! -f {dir}/env/ALLOW_NETS ] || export ALLOW_NETS=$(cat {dir}/env/ALLOW_NETS)\n\
         cd \"$ROOT\"\nexec setuidgid {user} {binary}\n",
        dir = shell_quote(&directory.to_string_lossy()),
        user = shell_quote(user),
        binary = shell_quote(&executable(binary)?.to_string_lossy()),
    ))
}

fn executable(name: &str) -> Result<PathBuf> {
    let current = std::env::current_exe()?;
    Ok(current
        .parent()
        .ok_or(Error::Format("configuration executable has no parent"))?
        .join(name))
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn write_file(path: &Path, contents: &[u8], mode: u32) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents)?;
    file.sync_all()?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(mode))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    fn directory(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "rgbdns-conf-{label}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    #[test]
    fn creates_tinydns_service_tree_without_overwriting() {
        let directory = directory("tinydns");
        let arguments = vec![
            "dns".into(),
            "log".into(),
            directory.to_string_lossy().into_owned(),
            "127.0.0.1".into(),
        ];
        configure(Service::Tinydns, &arguments).unwrap();
        assert!(directory.join("run").is_file());
        assert!(directory.join("root/data").is_file());
        assert!(directory.join("root/add-host").is_file());
        assert!(configure(Service::Tinydns, &arguments).is_err());
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn dnscache_tree_contains_current_hints_and_private_seed() {
        let directory = directory("dnscache");
        let arguments = vec![
            "dns".into(),
            "log".into(),
            directory.to_string_lossy().into_owned(),
        ];
        configure(Service::Dnscache, &arguments).unwrap();
        let hints = fs::read_to_string(directory.join("root/servers/@")).unwrap();
        assert!(hints.contains("A.ROOT-SERVERS.NET."));
        assert_eq!(fs::metadata(directory.join("seed")).unwrap().len(), 128);
        fs::remove_dir_all(directory).unwrap();
    }
}
