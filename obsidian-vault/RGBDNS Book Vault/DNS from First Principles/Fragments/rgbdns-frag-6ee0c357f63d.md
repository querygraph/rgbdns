---
type: "code-fragment"
fragment_id: "rgbdns-frag-6ee0c357f63d"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "query"
kind: "fn"
start_line: 487
end_line: 514
---

# query

- Fragment ID: `rgbdns-frag-6ee0c357f63d`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 487-514
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-6ee0c357f63d", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-6ee0c357f63d: fn query", "sourcePath": "src/server.rs", "startLine": 487, "endLine": 514}
```

## Excerpt

<span id="rgbdns-frag-6ee0c357f63d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6ee0c357f63d: fn query

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
