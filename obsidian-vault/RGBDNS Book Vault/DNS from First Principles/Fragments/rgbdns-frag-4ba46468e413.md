---
type: "code-fragment"
fragment_id: "rgbdns-frag-4ba46468e413"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "query"
kind: "fn"
start_line: 763
end_line: 789
---

# query

- Fragment ID: `rgbdns-frag-4ba46468e413`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 763-789
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4ba46468e413", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 763, "endLine": 789}
```

## Excerpt

<span id="rgbdns-frag-4ba46468e413" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4ba46468e413: fn query

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

```
