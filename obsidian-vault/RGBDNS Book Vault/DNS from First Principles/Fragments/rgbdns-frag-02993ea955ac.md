---
type: "code-fragment"
fragment_id: "rgbdns-frag-02993ea955ac"
source_path: "tests/cdb_golden.rs"
code_note: "DNS from First Principles/Code/tests/cdb_golden.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "pickdns_entries_match_original_c_golden"
kind: "fn"
start_line: 80
end_line: 89
---

# pickdns_entries_match_original_c_golden

- Fragment ID: `rgbdns-frag-02993ea955ac`
- Source file: [[DNS from First Principles/Code/tests/cdb_golden.rs.source|tests/cdb_golden.rs]]
- Lines: 80-89
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-02993ea955ac", "codeNote": "DNS from First Principles/Code/tests/cdb_golden.rs.source", "heading": "rgbdns-frag-02993ea955ac: fn pickdns_entries_match_original_c_golden", "sourcePath": "tests/cdb_golden.rs", "startLine": 80, "endLine": 89}
```

## Excerpt

<span id="rgbdns-frag-02993ea955ac" class="rgbdns-fragment-target"></span>
### rgbdns-frag-02993ea955ac: fn pickdns_entries_match_original_c_golden

```rust
fn pickdns_entries_match_original_c_golden() {
    let database = pick::Database::parse(include_str!("fixtures/pickdns-data")).unwrap();
    let path = path("pickdns");
    pick::compile(&database, &path).unwrap();
    assert_eq!(
        entries(&path),
        expected(include_str!("fixtures/pickdns-cdb.entries"))
    );
    fs::remove_file(path).unwrap();
}
```
