---
type: "code-fragment"
fragment_id: "rgbdns-frag-1649bad1a34b"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "acme_overlay_publishes_the_newer_of_source_and_stored_serials"
kind: "fn"
start_line: 1233
end_line: 1260
---

# acme_overlay_publishes_the_newer_of_source_and_stored_serials

- Fragment ID: `rgbdns-frag-1649bad1a34b`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 1233-1260
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1649bad1a34b", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 1233, "endLine": 1260}
```

## Excerpt

<span id="rgbdns-frag-1649bad1a34b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1649bad1a34b: fn acme_overlay_publishes_the_newer_of_source_and_stored_serials

```rust
    fn acme_overlay_publishes_the_newer_of_source_and_stored_serials() {
        let zone: Name = "example".parse().unwrap();
        let soa = |base: u32| {
            Zone::parse(&format!(
                "Zexample:ns.example:hostmaster.example:{base}:16384:2048:1048576:2560:3600\n"
            ))
            .unwrap()
        };
        let published = |base: u32, stored: u32| {
            let serials = BTreeMap::from([(zone.clone(), stored)]);
            soa(base)
                .with_acme_overlay(&[], &serials)
                .unwrap()
                .soa_serial(&zone)
                .unwrap()
        };
        // an operator's edit after the last ACME update must still publish
        assert_eq!(published(2_026_091_201, 2_026_081_503), 2_026_091_201);
        // ACME updates since the last edit keep their higher serial
        assert_eq!(published(2_026_081_501, 2_026_081_503), 2_026_081_503);
        assert_eq!(published(7, 7), 7);
        // RFC 1982 order across the wrap
        assert_eq!(published(2, u32::MAX), 2);
        assert_eq!(newer_serial(u32::MAX, 2), 2);
        assert_eq!(newer_serial(2, u32::MAX), 2);
    }

    #[test]
```
