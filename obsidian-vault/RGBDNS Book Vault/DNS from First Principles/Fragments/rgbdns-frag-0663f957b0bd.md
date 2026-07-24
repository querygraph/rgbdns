---
type: "code-fragment"
fragment_id: "rgbdns-frag-0663f957b0bd"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc6891_badvers_uses_the_extended_response_code"
kind: "fn"
start_line: 161
end_line: 169
---

# rfc6891_badvers_uses_the_extended_response_code

- Fragment ID: `rgbdns-frag-0663f957b0bd`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 161-169
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-0663f957b0bd", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-0663f957b0bd: fn rfc6891_badvers_uses_the_extended_response_code", "sourcePath": "tests/rfc_conformance.rs", "startLine": 161, "endLine": 169}
```

## Excerpt

<span id="rgbdns-frag-0663f957b0bd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0663f957b0bd: fn rfc6891_badvers_uses_the_extended_response_code

```rust
fn rfc6891_badvers_uses_the_extended_response_code() {
    let mut request = query("www.example", RecordType::A);
    request.additionals.push(opt(1232, 7, 0, Vec::new()));
    let response = response(&request);
    assert_eq!(extended_rcode(&response), 16);
    assert!(response.answers.is_empty());
}

#[test]
```
