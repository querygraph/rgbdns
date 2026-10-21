---
type: "code-fragment"
fragment_id: "rgbdns-frag-809b1f24c9a9"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "implicit_soa_uses_source_serial_and_generic_types_are_restricted"
kind: "fn"
start_line: 1261
end_line: 1273
---

# implicit_soa_uses_source_serial_and_generic_types_are_restricted

- Fragment ID: `rgbdns-frag-809b1f24c9a9`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 1261-1273
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-809b1f24c9a9", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 1261, "endLine": 1273}
```

## Excerpt

<span id="rgbdns-frag-809b1f24c9a9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-809b1f24c9a9: fn implicit_soa_uses_source_serial_and_generic_types_are_restricted

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
