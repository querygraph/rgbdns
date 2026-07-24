---
type: "code-fragment"
fragment_id: "rgbdns-frag-ae1791d3fe93"
source_path: "tests/cdb_golden.rs"
code_note: "DNS from First Principles/Code/tests/cdb_golden.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "path"
kind: "fn"
start_line: 8
end_line: 18
---

# path

- Fragment ID: `rgbdns-frag-ae1791d3fe93`
- Source file: [[DNS from First Principles/Code/tests/cdb_golden.rs.source|tests/cdb_golden.rs]]
- Lines: 8-18
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-ae1791d3fe93", "codeNote": "DNS from First Principles/Code/tests/cdb_golden.rs.source", "heading": "rgbdns-frag-ae1791d3fe93: fn path", "sourcePath": "tests/cdb_golden.rs", "startLine": 8, "endLine": 18}
```

## Excerpt

<span id="rgbdns-frag-ae1791d3fe93" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ae1791d3fe93: fn path

```rust
fn path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "rgbdns-golden-{label}-{}-{}.cdb",
        std::process::id(),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

```
