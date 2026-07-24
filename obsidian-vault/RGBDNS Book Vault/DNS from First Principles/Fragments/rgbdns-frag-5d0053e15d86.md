---
type: "code-fragment"
fragment_id: "rgbdns-frag-5d0053e15d86"
source_path: "src/pick.rs"
code_note: "DNS from First Principles/Code/src/pick.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "cdb_roundtrip_preserves_locations_and_addresses"
kind: "fn"
start_line: 279
end_line: 297
---

# cdb_roundtrip_preserves_locations_and_addresses

- Fragment ID: `rgbdns-frag-5d0053e15d86`
- Source file: [[DNS from First Principles/Code/src/pick.rs.source|src/pick.rs]]
- Lines: 279-297
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5d0053e15d86", "codeNote": "DNS from First Principles/Code/src/pick.rs.source", "heading": "rgbdns-frag-5d0053e15d86: fn cdb_roundtrip_preserves_locations_and_addresses", "sourcePath": "src/pick.rs", "startLine": 279, "endLine": 297}
```

## Excerpt

<span id="rgbdns-frag-5d0053e15d86" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5d0053e15d86: fn cdb_roundtrip_preserves_locations_and_addresses

```rust
    fn cdb_roundtrip_preserves_locations_and_addresses() {
        let database = Database::parse("%aa:192.0.2\n+www.example:192.0.2.1:aa\n").unwrap();
        let path = std::env::temp_dir().join(format!(
            "rgbdns-pick-{}-{}.cdb",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        compile(&database, &path).unwrap();
        let loaded = Database::from_file(&path).unwrap();
        fs::remove_file(path).unwrap();
        assert_eq!(
            loaded.addresses[&(*b"aa", "www.example".parse().unwrap())],
            [Ipv4Addr::new(192, 0, 2, 1)]
        );
    }
}
```
