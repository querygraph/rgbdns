---
type: "code-fragment"
fragment_id: "rgbdns-frag-c211f7926605"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc4343_name_comparison_is_ascii_case_insensitive"
kind: "fn"
start_line: 240
end_line: 249
---

# rfc4343_name_comparison_is_ascii_case_insensitive

- Fragment ID: `rgbdns-frag-c211f7926605`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 240-249
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-c211f7926605", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-c211f7926605: fn rfc4343_name_comparison_is_ascii_case_insensitive", "sourcePath": "tests/rfc_conformance.rs", "startLine": 240, "endLine": 249}
```

## Excerpt

<span id="rgbdns-frag-c211f7926605" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c211f7926605: fn rfc4343_name_comparison_is_ascii_case_insensitive

```rust
fn rfc4343_name_comparison_is_ascii_case_insensitive() {
    let upper: rgbdns::Name = "MiXeD.Example".parse().unwrap();
    let lower: rgbdns::Name = "mixed.example".parse().unwrap();
    assert_eq!(upper, lower);

    let answer = response(&query("WwW.Example", RecordType::A));
    assert_eq!(answer.answers[0].name.to_string(), "WwW.Example.");
}

#[test]
```
