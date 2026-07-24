---
type: "code-fragment"
fragment_id: "rgbdns-frag-1d93654dc774"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "encoder_compresses_repeated_names_and_suffixes"
kind: "fn"
start_line: 702
end_line: 731
---

# encoder_compresses_repeated_names_and_suffixes

- Fragment ID: `rgbdns-frag-1d93654dc774`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 702-731
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1d93654dc774", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-1d93654dc774: fn encoder_compresses_repeated_names_and_suffixes", "sourcePath": "src/packet.rs", "startLine": 702, "endLine": 731}
```

## Excerpt

<span id="rgbdns-frag-1d93654dc774" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1d93654dc774: fn encoder_compresses_repeated_names_and_suffixes

```rust
    fn encoder_compresses_repeated_names_and_suffixes() {
        let message = Message {
            questions: vec![Question {
                name: "www.deep.example".parse().unwrap(),
                qtype: RecordType::A,
                qclass: 1,
            }],
            answers: vec![
                Record {
                    name: "www.deep.example".parse().unwrap(),
                    ttl: 60,
                    data: RData::A("192.0.2.1".parse().unwrap()),
                },
                Record {
                    name: "mail.deep.example".parse().unwrap(),
                    ttl: 60,
                    data: RData::A("192.0.2.2".parse().unwrap()),
                },
            ],
            ..Message::default()
        };
        let wire = message.encode().unwrap();
        assert!(
            wire.windows(2)
                .any(|bytes| bytes[0] & 0xc0 == 0xc0 && bytes[1] == 12)
        );
        assert!(wire.len() < 80);
        assert_eq!(Message::decode(&wire).unwrap(), message);
    }
    #[test]
```
