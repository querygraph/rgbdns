---
type: "code-fragment"
fragment_id: "rgbdns-frag-e75608ea3a7a"
source_path: "tests/drill_interop.rs"
code_note: "DNS from First Principles/Code/tests/drill_interop.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "start_server"
kind: "fn"
start_line: 32
end_line: 62
---

# start_server

- Fragment ID: `rgbdns-frag-e75608ea3a7a`
- Source file: [[DNS from First Principles/Code/tests/drill_interop.rs.source|tests/drill_interop.rs]]
- Lines: 32-62
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-e75608ea3a7a", "codeNote": "DNS from First Principles/Code/tests/drill_interop.rs.source", "heading": "rgbdns-frag-e75608ea3a7a: fn start_server", "sourcePath": "tests/drill_interop.rs", "startLine": 32, "endLine": 62}
```

## Excerpt

<span id="rgbdns-frag-e75608ea3a7a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e75608ea3a7a: fn start_server

```rust
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

```
