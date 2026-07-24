---
type: "code-fragment"
fragment_id: "rgbdns-frag-d45c57d47515"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc6891_unknown_well_formed_options_are_ignored"
kind: "fn"
start_line: 146
end_line: 160
---

# rfc6891_unknown_well_formed_options_are_ignored

- Fragment ID: `rgbdns-frag-d45c57d47515`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 146-160
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-d45c57d47515", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-d45c57d47515: fn rfc6891_unknown_well_formed_options_are_ignored", "sourcePath": "tests/rfc_conformance.rs", "startLine": 146, "endLine": 160}
```

## Excerpt

<span id="rgbdns-frag-d45c57d47515" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d45c57d47515: fn rfc6891_unknown_well_formed_options_are_ignored

```rust
fn rfc6891_unknown_well_formed_options_are_ignored() {
    let mut request = query("www.example", RecordType::A);
    request
        .additionals
        .push(opt(1232, 0, 0, vec![0xfd, 0xe8, 0, 3, 1, 2, 3]));
    let response = response(&request);
    assert_eq!(rcode(&response), 0);
    assert_eq!(response.answers.len(), 1);
    assert!(matches!(
        &response.additionals.last().unwrap().data,
        RData::Opt { options, .. } if options.is_empty()
    ));
}

#[test]
```
