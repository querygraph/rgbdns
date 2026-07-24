---
type: "code-fragment"
fragment_id: "rgbdns-frag-7f35d0a0682b"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "structured_records_and_edns_roundtrip"
kind: "fn"
start_line: 654
end_line: 701
---

# structured_records_and_edns_roundtrip

- Fragment ID: `rgbdns-frag-7f35d0a0682b`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 654-701
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7f35d0a0682b", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-7f35d0a0682b: fn structured_records_and_edns_roundtrip", "sourcePath": "src/packet.rs", "startLine": 654, "endLine": 701}
```

## Excerpt

<span id="rgbdns-frag-7f35d0a0682b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7f35d0a0682b: fn structured_records_and_edns_roundtrip

```rust
    fn structured_records_and_edns_roundtrip() {
        let records = vec![
            Record {
                name: "example".parse().unwrap(),
                ttl: 60,
                data: RData::Soa {
                    mname: "ns.example".parse().unwrap(),
                    admin: "hostmaster.example".parse().unwrap(),
                    serial: 1,
                    refresh: 2,
                    retry: 3,
                    expire: 4,
                    minimum: 5,
                },
            },
            Record {
                name: "example".parse().unwrap(),
                ttl: 60,
                data: RData::Caa {
                    flags: 0,
                    tag: b"issue".to_vec(),
                    value: b"ca.example".to_vec(),
                },
            },
        ];
        let opt = Record {
            name: Name::root(),
            ttl: 0,
            data: RData::Opt {
                udp_payload: 1232,
                extended_rcode: 0,
                version: 0,
                flags: 0x8000,
                options: vec![0, 12, 0, 2, 0xaa, 0xbb],
            },
        };
        let message = Message {
            answers: records,
            additionals: vec![opt],
            ..Default::default()
        };
        assert_eq!(
            Message::decode(&message.encode().unwrap()).unwrap(),
            message
        );
    }

    #[test]
```
