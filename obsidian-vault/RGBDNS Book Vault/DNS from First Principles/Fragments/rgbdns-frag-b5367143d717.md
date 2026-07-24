---
type: "code-fragment"
fragment_id: "rgbdns-frag-b5367143d717"
source_path: "tests/cdb_golden.rs"
code_note: "DNS from First Principles/Code/tests/cdb_golden.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "tinydns_entries_match_patched_c_golden"
kind: "fn"
start_line: 56
end_line: 67
---

# tinydns_entries_match_patched_c_golden

- Fragment ID: `rgbdns-frag-b5367143d717`
- Source file: [[DNS from First Principles/Code/tests/cdb_golden.rs.source|tests/cdb_golden.rs]]
- Lines: 56-67
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-b5367143d717", "codeNote": "DNS from First Principles/Code/tests/cdb_golden.rs.source", "heading": "rgbdns-frag-b5367143d717: fn tinydns_entries_match_patched_c_golden", "sourcePath": "tests/cdb_golden.rs", "startLine": 56, "endLine": 67}
```

## Excerpt

<span id="rgbdns-frag-b5367143d717" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b5367143d717: fn tinydns_entries_match_patched_c_golden

```rust
fn tinydns_entries_match_patched_c_golden() {
    let database = Zone::parse(include_str!("fixtures/tinydns-data")).unwrap();
    let path = path("tinydns");
    cdb::compile(&database, &path).unwrap();
    assert_eq!(
        entries(&path),
        expected(include_str!("fixtures/tinydns-cdb.entries"))
    );
    fs::remove_file(path).unwrap();
}

#[test]
```
