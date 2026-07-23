#![cfg(unix)]

use nix::unistd::{Uid, User};
use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

fn directory() -> PathBuf {
    std::env::temp_dir().join(format!(
        "rgbdns-daemontools-test-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

#[test]
fn multilog_binary_timestamps_and_rotates() {
    let directory = directory();
    let mut child = Command::new(env!("CARGO_BIN_EXE_multilog"))
        .args(["t", "s40", "n2"])
        .arg(&directory)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"alpha\nbeta\ngamma\n")
        .unwrap();
    assert!(child.wait().unwrap().success());

    let entries = fs::read_dir(&directory)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    assert!(entries.contains(&"current".to_owned()));
    assert_eq!(
        entries
            .iter()
            .filter(|name| name.starts_with('@') && name.ends_with(".s"))
            .count(),
        2
    );
    assert!(entries.iter().all(|name| !name.contains(' ')));
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn setuidgid_replaces_itself_and_preserves_child_status() {
    let user = User::from_uid(Uid::effective()).unwrap().unwrap();
    let status = Command::new(env!("CARGO_BIN_EXE_setuidgid"))
        .arg(user.name)
        .args(["/bin/sh", "-c", "exit 7"])
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(7));
}
