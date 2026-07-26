---
type: "code-fragment"
fragment_id: "rgbdns-frag-474931ff0bc3"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "TAI_EPOCH"
kind: "const"
start_line: 902
end_line: 930
---

# TAI_EPOCH

- Fragment ID: `rgbdns-frag-474931ff0bc3`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 902-930
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-474931ff0bc3", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-474931ff0bc3: const TAI_EPOCH", "sourcePath": "src/zone.rs", "startLine": 902, "endLine": 930}
```

## Excerpt

<span id="rgbdns-frag-474931ff0bc3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-474931ff0bc3: const TAI_EPOCH

```rust
        const TAI_EPOCH: u64 = 4_611_686_018_427_387_914;
        let cutoff = format!("{:016x}", TAI_EPOCH + 200);
        let z = Zone::parse(&format!(
            ".example::ns.example\n\
             +expires.example:192.0.2.1:0:{cutoff}\n\
             +activates.example:192.0.2.2:60:{cutoff}\n"
        ))
        .unwrap();
        let expires = "expires.example".parse().unwrap();
        assert!(matches!(
            z.lookup_for(&expires, RecordType::A, None, 100),
            Lookup::Answer(records) if records[0].ttl == 100
        ));
        assert!(matches!(
            z.lookup_for(&expires, RecordType::A, None, 201),
            Lookup::NxDomain(_)
        ));
        let activates = "activates.example".parse().unwrap();
        assert!(matches!(
            z.lookup_for(&activates, RecordType::A, None, 200),
            Lookup::NxDomain(_)
        ));
        assert!(matches!(
            z.lookup_for(&activates, RecordType::A, None, 201),
            Lookup::Answer(records) if records[0].ttl == 60
        ));
    }

    #[test]
```
