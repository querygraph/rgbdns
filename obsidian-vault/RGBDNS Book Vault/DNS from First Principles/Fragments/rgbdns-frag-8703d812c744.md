---
type: "code-fragment"
fragment_id: "rgbdns-frag-8703d812c744"
source_path: "tests/cdb_golden.rs"
code_note: "DNS from First Principles/Code/tests/cdb_golden.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rbldns_entries_match_original_c_golden"
kind: "fn"
start_line: 68
end_line: 79
---

# rbldns_entries_match_original_c_golden

- Fragment ID: `rgbdns-frag-8703d812c744`
- Source file: [[DNS from First Principles/Code/tests/cdb_golden.rs.source|tests/cdb_golden.rs]]
- Lines: 68-79
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-8703d812c744", "codeNote": "DNS from First Principles/Code/tests/cdb_golden.rs.source", "heading": "rgbdns-frag-8703d812c744: fn rbldns_entries_match_original_c_golden", "sourcePath": "tests/cdb_golden.rs", "startLine": 68, "endLine": 79}
```

## Excerpt

<span id="rgbdns-frag-8703d812c744" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8703d812c744: fn rbldns_entries_match_original_c_golden

```rust
fn rbldns_entries_match_original_c_golden() {
    let database = rbl::Database::parse(include_str!("fixtures/rbldns-data")).unwrap();
    let path = path("rbldns");
    rbl::compile(&database, &path).unwrap();
    assert_eq!(
        entries(&path),
        expected(include_str!("fixtures/rbldns-cdb.entries"))
    );
    fs::remove_file(path).unwrap();
}

#[test]
```
