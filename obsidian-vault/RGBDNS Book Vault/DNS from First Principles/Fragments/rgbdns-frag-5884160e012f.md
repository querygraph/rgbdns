---
type: "code-fragment"
fragment_id: "rgbdns-frag-5884160e012f"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "signed_test_zone"
kind: "fn"
start_line: 790
end_line: 817
---

# signed_test_zone

- Fragment ID: `rgbdns-frag-5884160e012f`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 790-817
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5884160e012f", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 790, "endLine": 817}
```

## Excerpt

<span id="rgbdns-frag-5884160e012f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5884160e012f: fn signed_test_zone

```rust
    fn signed_test_zone() -> Zone {
        let directory =
            std::env::temp_dir().join(format!("rgbdns-server-dnssec-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let key = directory.join("example.pk8");
        let zone_name = "example".parse().unwrap();
        let policy = crate::dnssec::generate_key(&zone_name, &key).unwrap();
        let source = Zone::parse(
            "Zexample:ns.example:hostmaster.example:7:3600:600:86400:300:300\n\
             &example:192.0.2.53:ns.example:300\n\
             +www.example:192.0.2.1:300\n\
             +*.wild.example:192.0.2.2:300\n\
             &child.example:192.0.2.54:ns.child.example:300\n",
        )
        .unwrap();
        let records = crate::dnssec::sign_zone(&source, &policy).unwrap();
        std::fs::remove_dir_all(directory).unwrap();
        Zone::from_compiled_records(
            records
                .into_iter()
                .map(|record| (record, RecordMetadata::default()))
                .collect(),
            Vec::new(),
            Vec::new(),
        )
    }

    #[test]
```
