---
type: "code-fragment"
fragment_id: "rgbdns-frag-15a341490f0d"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "standard_axfr_does_not_expose_private_aname_metadata"
kind: "fn"
start_line: 569
end_line: 601
---

# standard_axfr_does_not_expose_private_aname_metadata

- Fragment ID: `rgbdns-frag-15a341490f0d`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 569-601
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-15a341490f0d", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 569, "endLine": 601}
```

## Excerpt

<span id="rgbdns-frag-15a341490f0d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-15a341490f0d: fn standard_axfr_does_not_expose_private_aname_metadata

```rust
    fn standard_axfr_does_not_expose_private_aname_metadata() {
        let zone = Zone::parse(
            "Zexample:ns.example:hostmaster.example:7:8:9:10:11:12\n\
             &example:192.0.2.53:ns.example:300\n\
             Aexample:publication.ghost.io:120\n",
        )
        .unwrap();
        let query = Message {
            id: 9,
            questions: vec![Question {
                name: "example".parse().unwrap(),
                qtype: RecordType::Axfr,
                qclass: 1,
            }],
            ..Default::default()
        };
        let responses = response_wires(&zone, query).unwrap();
        let records = responses
            .iter()
            .flat_map(|wire| Message::decode(wire).unwrap().answers)
            .collect::<Vec<_>>();
        assert!(
            !records
                .iter()
                .any(|record| { record.rr_type() == RecordType::Unknown(RGBDNS_ANAME_RECORD) })
        );
        assert!(!records.iter().any(|record| {
            record.name == "example".parse().unwrap()
                && matches!(record.rr_type(), RecordType::A | RecordType::Aaaa)
        }));
    }

    #[test]
```
