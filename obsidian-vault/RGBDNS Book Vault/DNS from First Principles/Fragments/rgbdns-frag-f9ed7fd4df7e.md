---
type: "code-fragment"
fragment_id: "rgbdns-frag-f9ed7fd4df7e"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "implicit_soa_uses_source_serial_and_generic_types_are_restricted"
kind: "fn"
start_line: 965
end_line: 977
---

# implicit_soa_uses_source_serial_and_generic_types_are_restricted

- Fragment ID: `rgbdns-frag-f9ed7fd4df7e`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 965-977
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f9ed7fd4df7e", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-f9ed7fd4df7e: fn implicit_soa_uses_source_serial_and_generic_types_are_restricted", "sourcePath": "src/zone.rs", "startLine": 965, "endLine": 977}
```

## Excerpt

<span id="rgbdns-frag-f9ed7fd4df7e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f9ed7fd4df7e: fn implicit_soa_uses_source_serial_and_generic_types_are_restricted

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
