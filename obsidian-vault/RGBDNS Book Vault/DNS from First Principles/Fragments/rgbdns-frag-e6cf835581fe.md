---
type: "code-fragment"
fragment_id: "rgbdns-frag-e6cf835581fe"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "longest_client_prefix_selects_location_records"
kind: "fn"
start_line: 931
end_line: 964
---

# longest_client_prefix_selects_location_records

- Fragment ID: `rgbdns-frag-e6cf835581fe`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 931-964
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e6cf835581fe", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-e6cf835581fe: fn longest_client_prefix_selects_location_records", "sourcePath": "src/zone.rs", "startLine": 931, "endLine": 964}
```

## Excerpt

<span id="rgbdns-frag-e6cf835581fe" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e6cf835581fe: fn longest_client_prefix_selects_location_records

```rust
    fn longest_client_prefix_selects_location_records() {
        let z = Zone::parse(
            ".example::ns.example\n\
             %aa:192\n\
             %bb:192.0.2\n\
             +located.example:192.0.2.1:60::aa\n\
             +located.example:192.0.2.2:60::bb\n",
        )
        .unwrap();
        let name = "located.example".parse().unwrap();
        assert!(matches!(
            z.lookup_for(
                &name,
                RecordType::A,
                Some(IpAddr::V4(Ipv4Addr::new(192, 0, 2, 55))),
                0,
            ),
            Lookup::Answer(records)
                if records.len() == 1
                    && records[0].data == RData::A(Ipv4Addr::new(192, 0, 2, 2))
        ));
        assert!(matches!(
            z.lookup_for(
                &name,
                RecordType::A,
                Some(IpAddr::V4(Ipv4Addr::new(192, 9, 9, 9))),
                0,
            ),
            Lookup::Answer(records)
                if records[0].data == RData::A(Ipv4Addr::new(192, 0, 2, 1))
        ));
    }

    #[test]
```
