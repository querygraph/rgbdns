---
type: "code-fragment"
fragment_id: "rgbdns-frag-c292ee23c8bf"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "explicit_soa_uses_ttl_field_nine"
kind: "fn"
start_line: 808
end_line: 825
---

# explicit_soa_uses_ttl_field_nine

- Fragment ID: `rgbdns-frag-c292ee23c8bf`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 808-825
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c292ee23c8bf", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-c292ee23c8bf: fn explicit_soa_uses_ttl_field_nine", "sourcePath": "src/zone.rs", "startLine": 808, "endLine": 825}
```

## Excerpt

<span id="rgbdns-frag-c292ee23c8bf" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c292ee23c8bf: fn explicit_soa_uses_ttl_field_nine

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
