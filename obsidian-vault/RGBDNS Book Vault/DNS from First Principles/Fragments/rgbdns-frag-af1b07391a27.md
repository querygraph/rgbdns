---
type: "code-fragment"
fragment_id: "rgbdns-frag-af1b07391a27"
source_path: "src/cdb.rs"
code_note: "DNS from First Principles/Code/src/cdb.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "rejects_truncated_database"
kind: "fn"
start_line: 325
end_line: 331
---

# rejects_truncated_database

- Fragment ID: `rgbdns-frag-af1b07391a27`
- Source file: [[DNS from First Principles/Code/src/cdb.rs.source|src/cdb.rs]]
- Lines: 325-331
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-af1b07391a27", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-af1b07391a27: fn rejects_truncated_database", "sourcePath": "src/cdb.rs", "startLine": 325, "endLine": 331}
```

## Excerpt

<span id="rgbdns-frag-af1b07391a27" class="rgbdns-fragment-target"></span>
### rgbdns-frag-af1b07391a27: fn rejects_truncated_database

```rust
    fn rejects_truncated_database() {
        let path = std::env::temp_dir().join(format!("rgbdns-short-{}.cdb", std::process::id()));
        fs::write(&path, [0; 20]).unwrap();
        assert!(load(&path).is_err());
        fs::remove_file(path).unwrap();
    }
}
```
