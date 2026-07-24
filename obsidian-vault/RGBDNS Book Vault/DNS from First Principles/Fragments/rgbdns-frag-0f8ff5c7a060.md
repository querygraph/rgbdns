---
type: "code-fragment"
fragment_id: "rgbdns-frag-0f8ff5c7a060"
source_path: "tests/dnscache_network.rs"
code_note: "DNS from First Principles/Code/tests/dnscache_network.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "validates_secure_and_rejects_bogus_dnssec"
kind: "fn"
start_line: 48
end_line: 72
---

# validates_secure_and_rejects_bogus_dnssec

- Fragment ID: `rgbdns-frag-0f8ff5c7a060`
- Source file: [[DNS from First Principles/Code/tests/dnscache_network.rs.source|tests/dnscache_network.rs]]
- Lines: 48-72
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-0f8ff5c7a060", "codeNote": "DNS from First Principles/Code/tests/dnscache_network.rs.source", "heading": "rgbdns-frag-0f8ff5c7a060: fn validates_secure_and_rejects_bogus_dnssec", "sourcePath": "tests/dnscache_network.rs", "startLine": 48, "endLine": 72}
```

## Excerpt

<span id="rgbdns-frag-0f8ff5c7a060" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0f8ff5c7a060: fn validates_secure_and_rejects_bogus_dnssec

```rust
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
```
