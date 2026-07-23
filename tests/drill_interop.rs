use std::{
    fs,
    net::TcpListener,
    path::PathBuf,
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, SystemTime},
};

struct Server {
    child: Child,
    data: PathBuf,
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = fs::remove_file(&self.data);
    }
}

fn drill_available() -> bool {
    Command::new("drill")
        .arg("-v")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

fn start_server() -> (Server, u16) {
    let reservation = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = reservation.local_addr().unwrap().port();
    drop(reservation);

    let unique = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let data =
        std::env::temp_dir().join(format!("rgbdns-drill-{}-{unique}.data", std::process::id()));
    fs::write(
        &data,
        ".example:192.0.2.53:ns.example:300\n\
         +www.example:192.0.2.1:300\n\
         3www.example:20010db8000000000000000000000001:300\n\
         'txt.example:hello:300\n",
    )
    .unwrap();
    let child = Command::new(env!("CARGO_BIN_EXE_tinydns"))
        .env("IP", "127.0.0.1")
        .env("PORT", port.to_string())
        .env("DATA", &data)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    thread::sleep(Duration::from_millis(250));
    (Server { child, data }, port)
}

fn drill(port: u16, extra: &[&str], name: &str, record_type: &str) -> String {
    let output = Command::new("drill")
        .args(extra)
        .args([
            "-p",
            &port.to_string(),
            name,
            "@127.0.0.1",
            record_type,
            "IN",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "drill failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn ldns_drill_interoperates_over_udp_tcp_edns_and_unknown_types() {
    if !drill_available() {
        eprintln!("skipping: ldns drill is not installed");
        return;
    }
    let (_server, port) = start_server();

    let udp = drill(port, &["-u"], "WwW.ExAmPlE", "A");
    assert!(udp.contains("192.0.2.1"));
    assert!(udp.contains("rcode: NOERROR"));

    let tcp = drill(port, &["-t"], "www.example", "AAAA");
    assert!(tcp.contains("2001:db8::1"));

    let edns = drill(port, &["-D", "-b", "1232"], "txt.example", "TXT");
    assert!(edns.contains("\"hello\""));
    assert!(edns.contains("EDNS"));

    let unknown = drill(port, &[], "www.example", "TYPE65000");
    assert!(unknown.contains("rcode: NOERROR"));
    assert!(unknown.contains("ANSWER: 0"));
}
