---
type: "code-fragment"
fragment_id: "rgbdns-frag-91fad3d9554d"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "TAI_EPOCH"
kind: "const"
start_line: 832
end_line: 860
---

# TAI_EPOCH

- Fragment ID: `rgbdns-frag-91fad3d9554d`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 832-860
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-91fad3d9554d", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-91fad3d9554d: const TAI_EPOCH", "sourcePath": "src/zone.rs", "startLine": 832, "endLine": 860}
```

## Excerpt

<span id="rgbdns-frag-91fad3d9554d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-91fad3d9554d: const TAI_EPOCH

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
