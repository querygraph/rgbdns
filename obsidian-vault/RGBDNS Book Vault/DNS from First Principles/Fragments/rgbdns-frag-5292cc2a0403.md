---
type: "code-fragment"
fragment_id: "rgbdns-frag-5292cc2a0403"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc1035_response_identity_flags_and_question_are_coherent"
kind: "fn"
start_line: 7
end_line: 22
---

# rfc1035_response_identity_flags_and_question_are_coherent

- Fragment ID: `rgbdns-frag-5292cc2a0403`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 7-22
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-5292cc2a0403", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-5292cc2a0403: fn rfc1035_response_identity_flags_and_question_are_coherent", "sourcePath": "tests/rfc_conformance.rs", "startLine": 7, "endLine": 22}
```

## Excerpt

<span id="rgbdns-frag-5292cc2a0403" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5292cc2a0403: fn rfc1035_response_identity_flags_and_question_are_coherent

```rust
fn rfc1035_response_identity_flags_and_question_are_coherent() {
    let request = query("WwW.ExAmPlE", RecordType::A);
    let response = response(&request);

    assert_eq!(response.id, ID);
    assert_ne!(response.flags & 0x8000, 0, "QR");
    assert_ne!(response.flags & 0x0400, 0, "AA");
    assert_eq!(response.flags & 0x0200, 0, "TC");
    assert_ne!(response.flags & 0x0100, 0, "RD is copied");
    assert_eq!(response.flags & 0x0080, 0, "authorities do not offer RA");
    assert_eq!(response.flags & 0x0070, 0, "reserved Z bits");
    assert_eq!(response.questions, request.questions);
    assert_eq!(response.questions[0].name.to_string(), "WwW.ExAmPlE.");
}

#[test]
```
