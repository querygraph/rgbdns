---
type: "code-fragment"
fragment_id: "rgbdns-frag-3abbe7dfdfde"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc2181_referral_contains_only_in_bailiwick_glue"
kind: "fn"
start_line: 266
end_line: 279
---

# rfc2181_referral_contains_only_in_bailiwick_glue

- Fragment ID: `rgbdns-frag-3abbe7dfdfde`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 266-279
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-3abbe7dfdfde", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-3abbe7dfdfde: fn rfc2181_referral_contains_only_in_bailiwick_glue", "sourcePath": "tests/rfc_conformance.rs", "startLine": 266, "endLine": 279}
```

## Excerpt

<span id="rgbdns-frag-3abbe7dfdfde" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3abbe7dfdfde: fn rfc2181_referral_contains_only_in_bailiwick_glue

```rust
fn rfc2181_referral_contains_only_in_bailiwick_glue() {
    let request = query("host.child.example", RecordType::A);
    let response = response(&request);
    assert_eq!(response.flags & 0x0400, 0);
    assert_eq!(response.authorities.len(), 1);
    assert!(response.additionals.iter().all(|record| {
        record
            .name
            .is_subdomain_of(&"child.example".parse().unwrap())
            && matches!(record.rr_type(), RecordType::A | RecordType::Aaaa)
    }));
}

#[test]
```
