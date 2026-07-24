---
type: "code-fragment"
fragment_id: "rgbdns-frag-7da8254d757f"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc1035_unsupported_query_class_gets_notimp"
kind: "fn"
start_line: 92
end_line: 99
---

# rfc1035_unsupported_query_class_gets_notimp

- Fragment ID: `rgbdns-frag-7da8254d757f`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 92-99
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-7da8254d757f", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-7da8254d757f: fn rfc1035_unsupported_query_class_gets_notimp", "sourcePath": "tests/rfc_conformance.rs", "startLine": 92, "endLine": 99}
```

## Excerpt

<span id="rgbdns-frag-7da8254d757f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7da8254d757f: fn rfc1035_unsupported_query_class_gets_notimp

```rust
fn rfc1035_unsupported_query_class_gets_notimp() {
    let mut request = query("www.example", RecordType::A);
    request.questions[0].qclass = 3;
    let response = response(&request);
    assert_eq!(rcode(&response), 4);
}

#[test]
```
