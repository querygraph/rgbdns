---
type: "code-fragment"
fragment_id: "rgbdns-frag-3a4596180b49"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "explicit_soa_uses_ttl_field_nine"
kind: "fn"
start_line: 1146
end_line: 1163
---

# explicit_soa_uses_ttl_field_nine

- Fragment ID: `rgbdns-frag-3a4596180b49`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 1146-1163
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3a4596180b49", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 1146, "endLine": 1163}
```

## Excerpt

<span id="rgbdns-frag-3a4596180b49" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3a4596180b49: fn explicit_soa_uses_ttl_field_nine

```rust
    fn explicit_soa_uses_ttl_field_nine() {
        let z = Zone::parse("Zexample:ns.example:hostmaster.example:7:8:9:10:11:12\n").unwrap();
        assert!(matches!(
            z.lookup(&"example".parse().unwrap(), RecordType::Soa),
            Lookup::Answer(records)
                if records[0].ttl == 12
                    && matches!(&records[0].data, RData::Soa {
                        serial: 7,
                        refresh: 8,
                        retry: 9,
                        expire: 10,
                        minimum: 11,
                        ..
                    })
        ));
    }

    #[test]
```
