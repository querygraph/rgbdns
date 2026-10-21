---
type: "code-fragment"
fragment_id: "rgbdns-frag-b5c42adc662d"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "structured_records_and_edns_roundtrip"
kind: "fn"
start_line: 810
end_line: 857
---

# structured_records_and_edns_roundtrip

- Fragment ID: `rgbdns-frag-b5c42adc662d`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 810-857
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b5c42adc662d", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 810, "endLine": 857}
```

## Excerpt

<span id="rgbdns-frag-b5c42adc662d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b5c42adc662d: fn structured_records_and_edns_roundtrip

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
