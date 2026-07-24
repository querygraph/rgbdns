---
type: "code-fragment"
fragment_id: "rgbdns-frag-41b924ae97ce"
source_path: "tests/dnscache_network.rs"
code_note: "DNS from First Principles/Code/tests/dnscache_network.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "query"
kind: "fn"
start_line: 16
end_line: 47
---

# query

- Fragment ID: `rgbdns-frag-41b924ae97ce`
- Source file: [[DNS from First Principles/Code/tests/dnscache_network.rs.source|tests/dnscache_network.rs]]
- Lines: 16-47
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-41b924ae97ce", "codeNote": "DNS from First Principles/Code/tests/dnscache_network.rs.source", "heading": "rgbdns-frag-41b924ae97ce: fn query", "sourcePath": "tests/dnscache_network.rs", "startLine": 16, "endLine": 47}
```

## Excerpt

<span id="rgbdns-frag-41b924ae97ce" class="rgbdns-fragment-target"></span>
### rgbdns-frag-41b924ae97ce: fn query

```rust
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
```
