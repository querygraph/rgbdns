---
type: "code-fragment"
fragment_id: "rgbdns-frag-e4a8c49fe363"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc8906_unknown_header_flags_do_not_suppress_or_taint_answers"
kind: "fn"
start_line: 66
end_line: 78
---

# rfc8906_unknown_header_flags_do_not_suppress_or_taint_answers

- Fragment ID: `rgbdns-frag-e4a8c49fe363`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 66-78
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-e4a8c49fe363", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-e4a8c49fe363: fn rfc8906_unknown_header_flags_do_not_suppress_or_taint_answers", "sourcePath": "tests/rfc_conformance.rs", "startLine": 66, "endLine": 78}
```

## Excerpt

<span id="rgbdns-frag-e4a8c49fe363" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e4a8c49fe363: fn rfc8906_unknown_header_flags_do_not_suppress_or_taint_answers

```rust
fn rfc8906_unknown_header_flags_do_not_suppress_or_taint_answers() {
    for flags in [0x0020, 0x0010, 0x0040, 0x0070, 0x0120, 0x0170] {
        let mut request = query("www.example", RecordType::A);
        request.flags = flags;
        let response = response(&request);
        assert_eq!(rcode(&response), 0, "request flags {flags:#06x}");
        assert_eq!(response.answers.len(), 1);
        assert_eq!(response.flags & 0x0070, 0);
        assert_eq!(response.flags & 0x0100, flags & 0x0100);
    }
}

#[test]
```
