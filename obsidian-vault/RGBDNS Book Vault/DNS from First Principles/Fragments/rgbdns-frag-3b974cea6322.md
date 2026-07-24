---
type: "code-fragment"
fragment_id: "rgbdns-frag-3b974cea6322"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc2181_cname_cannot_coexist_with_other_data_or_multiple_targets"
kind: "fn"
start_line: 296
end_line: 313
---

# rfc2181_cname_cannot_coexist_with_other_data_or_multiple_targets

- Fragment ID: `rgbdns-frag-3b974cea6322`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 296-313
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-3b974cea6322", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-3b974cea6322: fn rfc2181_cname_cannot_coexist_with_other_data_or_multiple_targets", "sourcePath": "tests/rfc_conformance.rs", "startLine": 296, "endLine": 313}
```

## Excerpt

<span id="rgbdns-frag-3b974cea6322" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3b974cea6322: fn rfc2181_cname_cannot_coexist_with_other_data_or_multiple_targets

```rust
fn rfc2181_cname_cannot_coexist_with_other_data_or_multiple_targets() {
    assert!(
        rgbdns::zone::Zone::parse(
            ".example::ns.example\n\
             Calias.example:first.example\n\
             +alias.example:192.0.2.1\n",
        )
        .is_err()
    );
    assert!(
        rgbdns::zone::Zone::parse(
            ".example::ns.example\n\
             Calias.example:first.example\n\
             Calias.example:second.example\n",
        )
        .is_err()
    );
}
```
