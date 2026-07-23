use std::{
    net::{TcpListener, UdpSocket},
    process::{Child, Command, Stdio},
    thread,
    time::Duration,
};

struct CacheProcess(Child);
impl Drop for CacheProcess {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn query(port: u16, name: &str) -> Vec<u8> {
    let mut wire = vec![0x12, 0x34, 0x01, 0, 0, 1, 0, 0, 0, 0, 0, 1];
    for label in name.split('.') {
        wire.push(label.len() as u8);
        wire.extend(label.as_bytes());
    }
    wire.extend([0, 0, 1, 0, 1]);
    // EDNS(0), 1232-byte payload, DO bit.
    wire.extend([0, 0, 41, 0x04, 0xd0, 0, 0, 0x80, 0, 0, 0]);

    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    socket
        .set_read_timeout(Some(Duration::from_secs(10)))
        .unwrap();
    socket.send_to(&wire, ("127.0.0.1", port)).unwrap();
    let mut response = vec![0; 65_535];
    let size = loop {
        match socket.recv(&mut response) {
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            result => break result.unwrap(),
        }
    };
    response.truncate(size);
    response
}

/// This is excluded from default offline builds because it deliberately
/// contacts the public DNS hierarchy. Run with:
///
/// `cargo test --test dnscache_network -- --ignored`
#[test]
#[ignore = "requires direct outbound UDP/TCP DNS"]
fn validates_secure_and_rejects_bogus_dnssec() {
    let reservation = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = reservation.local_addr().unwrap().port();
    drop(reservation);
    let child = Command::new(env!("CARGO_BIN_EXE_dnscache"))
        .env("IP", "127.0.0.1")
        .env("PORT", port.to_string())
        .env("ROOTS", "config/root.hints")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let _guard = CacheProcess(child);
    thread::sleep(Duration::from_millis(500));

    let secure = query(port, "cloudflare.com");
    let secure_flags = u16::from_be_bytes([secure[2], secure[3]]);
    assert_eq!(secure_flags & 0xf, 0);
    assert_ne!(secure_flags & 0x20, 0, "validated answer must set AD");

    let bogus = query(port, "dnssec-failed.org");
    let bogus_flags = u16::from_be_bytes([bogus[2], bogus[3]]);
    assert_eq!(bogus_flags & 0xf, 2, "bogus DNSSEC must return SERVFAIL");
    assert_eq!(bogus_flags & 0x20, 0, "bogus DNSSEC must not set AD");
}
