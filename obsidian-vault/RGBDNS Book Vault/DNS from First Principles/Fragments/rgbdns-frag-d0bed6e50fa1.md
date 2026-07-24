---
type: "code-fragment"
fragment_id: "rgbdns-frag-d0bed6e50fa1"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "query_roundtrip"
kind: "fn"
start_line: 613
end_line: 626
---

# query_roundtrip

- Fragment ID: `rgbdns-frag-d0bed6e50fa1`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 613-626
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d0bed6e50fa1", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-d0bed6e50fa1: fn query_roundtrip", "sourcePath": "src/packet.rs", "startLine": 613, "endLine": 626}
```

## Excerpt

<span id="rgbdns-frag-d0bed6e50fa1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d0bed6e50fa1: fn query_roundtrip

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
