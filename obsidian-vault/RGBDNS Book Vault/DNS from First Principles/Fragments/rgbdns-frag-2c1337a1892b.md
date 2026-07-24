---
type: "code-fragment"
fragment_id: "rgbdns-frag-2c1337a1892b"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "legacy_udp_is_limited_to_512_and_keeps_whole_records"
kind: "fn"
start_line: 573
end_line: 587
---

# legacy_udp_is_limited_to_512_and_keeps_whole_records

- Fragment ID: `rgbdns-frag-2c1337a1892b`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 573-587
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-2c1337a1892b", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-2c1337a1892b: fn legacy_udp_is_limited_to_512_and_keeps_whole_records", "sourcePath": "src/server.rs", "startLine": 573, "endLine": 587}
```

## Excerpt

<span id="rgbdns-frag-2c1337a1892b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2c1337a1892b: fn legacy_udp_is_limited_to_512_and_keeps_whole_records

```rust
    fn legacy_udp_is_limited_to_512_and_keeps_whole_records() {
        let mut data = ".example::ns.example\n".to_owned();
        for index in 0..80 {
            data.push_str(&format!("+many.example:192.0.2.{}\n", index % 250 + 1));
        }
        let zone = Zone::parse(&data).unwrap();
        let wire = respond(&zone, &query("many.example", RecordType::A, None), 4096).unwrap();
        assert!(wire.len() <= 512);
        let response = Message::decode(&wire).unwrap();
        assert_ne!(response.flags & 0x0200, 0);
        assert!(!response.answers.is_empty());
        assert!(response.answers.len() < 80);
    }

    #[test]
```
