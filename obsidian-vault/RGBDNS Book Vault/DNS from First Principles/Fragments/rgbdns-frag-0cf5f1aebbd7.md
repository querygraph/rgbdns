---
type: "code-fragment"
fragment_id: "rgbdns-frag-0cf5f1aebbd7"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "query"
kind: "fn"
start_line: 379
end_line: 406
---

# query

- Fragment ID: `rgbdns-frag-0cf5f1aebbd7`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 379-406
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0cf5f1aebbd7", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-0cf5f1aebbd7: fn query", "sourcePath": "src/server.rs", "startLine": 379, "endLine": 406}
```

## Excerpt

<span id="rgbdns-frag-0cf5f1aebbd7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0cf5f1aebbd7: fn query

```rust
    fn query(name: &str, typ: RecordType, opt: Option<(u16, u8)>) -> Vec<u8> {
        let mut message = Message {
            id: 0x1234,
            flags: 0x0100,
            questions: vec![Question {
                name: name.parse().unwrap(),
                qtype: typ,
                qclass: 1,
            }],
            ..Default::default()
        };
        if let Some((payload, version)) = opt {
            message.additionals.push(Record {
                name: Name::root(),
                ttl: 0,
                data: RData::Opt {
                    udp_payload: payload,
                    extended_rcode: 0,
                    version,
                    flags: 0x8000,
                    options: Vec::new(),
                },
            });
        }
        message.encode().unwrap()
    }

    #[test]
```
