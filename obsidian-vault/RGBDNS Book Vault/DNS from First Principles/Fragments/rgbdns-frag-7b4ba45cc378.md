---
type: "code-fragment"
fragment_id: "rgbdns-frag-7b4ba45cc378"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "implicit_soa_uses_source_serial_and_generic_types_are_restricted"
kind: "fn"
start_line: 895
end_line: 907
---

# implicit_soa_uses_source_serial_and_generic_types_are_restricted

- Fragment ID: `rgbdns-frag-7b4ba45cc378`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 895-907
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7b4ba45cc378", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-7b4ba45cc378: fn implicit_soa_uses_source_serial_and_generic_types_are_restricted", "sourcePath": "src/zone.rs", "startLine": 895, "endLine": 907}
```

## Excerpt

<span id="rgbdns-frag-7b4ba45cc378" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7b4ba45cc378: fn implicit_soa_uses_source_serial_and_generic_types_are_restricted

```rust
    fn implicit_soa_uses_source_serial_and_generic_types_are_restricted() {
        let zone = Zone::parse_with_serial(".example::ns.example\n", 1_234_567).unwrap();
        assert!(matches!(
            zone.lookup(&"example".parse().unwrap(), RecordType::Soa),
            Lookup::Answer(records)
                if matches!(records[0].data, RData::Soa { serial: 1_234_567, .. })
        ));
        for record_type in [0, 2, 5, 6, 12, 15, 252] {
            assert!(Zone::parse(&format!(":example:{record_type}:x\n")).is_err());
        }
    }

    #[test]
```
