---
type: "code-fragment"
fragment_id: "rgbdns-frag-b2c41a84e938"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc8906_unknown_types_use_name_existence_not_notimp"
kind: "fn"
start_line: 79
end_line: 91
---

# rfc8906_unknown_types_use_name_existence_not_notimp

- Fragment ID: `rgbdns-frag-b2c41a84e938`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 79-91
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-b2c41a84e938", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-b2c41a84e938: fn rfc8906_unknown_types_use_name_existence_not_notimp", "sourcePath": "tests/rfc_conformance.rs", "startLine": 79, "endLine": 91}
```

## Excerpt

<span id="rgbdns-frag-b2c41a84e938" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b2c41a84e938: fn rfc8906_unknown_types_use_name_existence_not_notimp

```rust
fn rfc8906_unknown_types_use_name_existence_not_notimp() {
    let existing = response(&query("www.example", RecordType::Unknown(65_000)));
    assert_eq!(rcode(&existing), 0);
    assert!(existing.answers.is_empty());
    assert_eq!(existing.authorities[0].rr_type(), RecordType::Soa);

    let absent = response(&query("absent.example", RecordType::Unknown(65_000)));
    assert_eq!(rcode(&absent), 3);
    assert!(absent.answers.is_empty());
    assert_eq!(absent.authorities[0].rr_type(), RecordType::Soa);
}

#[test]
```
