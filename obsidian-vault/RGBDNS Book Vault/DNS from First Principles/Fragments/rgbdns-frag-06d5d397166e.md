---
type: "code-fragment"
fragment_id: "rgbdns-frag-06d5d397166e"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "query_roundtrip"
kind: "fn"
start_line: 769
end_line: 782
---

# query_roundtrip

- Fragment ID: `rgbdns-frag-06d5d397166e`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 769-782
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-06d5d397166e", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 769, "endLine": 782}
```

## Excerpt

<span id="rgbdns-frag-06d5d397166e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-06d5d397166e: fn query_roundtrip

```rust
    fn query_roundtrip() {
        let m = Message {
            id: 42,
            flags: 0x100,
            questions: vec![Question {
                name: "Example.COM".parse().unwrap(),
                qtype: RecordType::A,
                qclass: 1,
            }],
            ..Default::default()
        };
        assert_eq!(Message::decode(&m.encode().unwrap()).unwrap(), m)
    }
    #[test]
```
