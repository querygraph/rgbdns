---
type: "code-fragment"
fragment_id: "rgbdns-frag-bd13b0414c83"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc6891_advertised_udp_limit_is_honored_with_parseable_truncation"
kind: "fn"
start_line: 204
end_line: 222
---

# rfc6891_advertised_udp_limit_is_honored_with_parseable_truncation

- Fragment ID: `rgbdns-frag-bd13b0414c83`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 204-222
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-bd13b0414c83", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-bd13b0414c83: fn rfc6891_advertised_udp_limit_is_honored_with_parseable_truncation", "sourcePath": "tests/rfc_conformance.rs", "startLine": 204, "endLine": 222}
```

## Excerpt

<span id="rgbdns-frag-bd13b0414c83" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bd13b0414c83: fn rfc6891_advertised_udp_limit_is_honored_with_parseable_truncation

```rust
fn rfc6891_advertised_udp_limit_is_honored_with_parseable_truncation() {
    let mut data = ".example::ns.example\n".to_owned();
    for index in 1..=200 {
        data.push_str(&format!("+many.example:192.0.2.{}:300\n", index % 250));
    }
    let zone = rgbdns::zone::Zone::parse(&data).unwrap();
    let mut request = query("many.example", RecordType::A);
    request.additionals.push(opt(768, 0, 0x8000, Vec::new()));
    let wire = rgbdns::server::respond(&zone, &request.encode().unwrap(), 4096).unwrap();
    assert!(wire.len() <= 768);
    let response = Message::decode(&wire).unwrap();
    assert_ne!(response.flags & 0x0200, 0);
    assert!(matches!(
        response.additionals.last().map(|record| &record.data),
        Some(RData::Opt { .. })
    ));
}

#[test]
```
