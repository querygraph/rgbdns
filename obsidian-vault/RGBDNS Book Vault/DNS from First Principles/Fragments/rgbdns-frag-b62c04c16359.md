---
type: "code-fragment"
fragment_id: "rgbdns-frag-b62c04c16359"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "original_ns_and_mx_field_positions_and_expansion"
kind: "fn"
start_line: 845
end_line: 877
---

# original_ns_and_mx_field_positions_and_expansion

- Fragment ID: `rgbdns-frag-b62c04c16359`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 845-877
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b62c04c16359", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-b62c04c16359: fn original_ns_and_mx_field_positions_and_expansion", "sourcePath": "src/zone.rs", "startLine": 845, "endLine": 877}
```

## Excerpt

<span id="rgbdns-frag-b62c04c16359" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b62c04c16359: fn original_ns_and_mx_field_positions_and_expansion

```rust
    fn original_ns_and_mx_field_positions_and_expansion() {
        let z = Zone::parse(
            ".example:192.0.2.53:a:300\n\
             @example:192.0.2.25:mail:20:400\n",
        )
        .unwrap();
        assert!(matches!(
            z.lookup(&"example".parse().unwrap(), RecordType::Ns),
            Lookup::Answer(records)
                if records[0].ttl == 300
                    && matches!(&records[0].data, RData::Name(_, target)
                        if target.to_string() == "a.ns.example.")
        ));
        assert!(matches!(
            z.lookup(&"example".parse().unwrap(), RecordType::Mx),
            Lookup::Answer(records)
                if records[0].ttl == 400
                    && matches!(&records[0].data, RData::Mx(20, target)
                        if target.to_string() == "mail.mx.example.")
        ));
        assert!(matches!(
            z.lookup(&"mail.mx.example".parse().unwrap(), RecordType::A),
            Lookup::Answer(records)
                if records[0].ttl == 400
                    && records[0].data == RData::A(Ipv4Addr::new(192, 0, 2, 25))
        ));
        assert!(matches!(
            z.lookup(&"example".parse().unwrap(), RecordType::Soa),
            Lookup::Answer(records) if records[0].ttl == 2560
        ));
    }

    #[test]
```
