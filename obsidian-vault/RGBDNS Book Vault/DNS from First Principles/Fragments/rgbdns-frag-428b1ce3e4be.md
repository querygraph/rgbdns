---
type: "code-fragment"
fragment_id: "rgbdns-frag-428b1ce3e4be"
source_path: "src/cdb.rs"
code_note: "DNS from First Principles/Code/src/cdb.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "rejects_truncated_database"
kind: "fn"
start_line: 365
end_line: 371
---

# rejects_truncated_database

- Fragment ID: `rgbdns-frag-428b1ce3e4be`
- Source file: [[DNS from First Principles/Code/src/cdb.rs.source|src/cdb.rs]]
- Lines: 365-371
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-428b1ce3e4be", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-428b1ce3e4be: fn rejects_truncated_database", "sourcePath": "src/cdb.rs", "startLine": 365, "endLine": 371}
```

## Excerpt

<span id="rgbdns-frag-428b1ce3e4be" class="rgbdns-fragment-target"></span>
### rgbdns-frag-428b1ce3e4be: fn rejects_truncated_database

```rust
    fn rejects_truncated_database() {
        let path = std::env::temp_dir().join(format!("rgbdns-short-{}.cdb", std::process::id()));
        fs::write(&path, [0; 20]).unwrap();
        assert!(load(&path).is_err());
        fs::remove_file(path).unwrap();
    }
}
```
