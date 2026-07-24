---
type: "code-fragment"
fragment_id: "rgbdns-frag-114e43800a7f"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc9619_standard_queries_require_exactly_one_question"
kind: "fn"
start_line: 23
end_line: 37
---

# rfc9619_standard_queries_require_exactly_one_question

- Fragment ID: `rgbdns-frag-114e43800a7f`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 23-37
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-114e43800a7f", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-114e43800a7f: fn rfc9619_standard_queries_require_exactly_one_question", "sourcePath": "tests/rfc_conformance.rs", "startLine": 23, "endLine": 37}
```

## Excerpt

<span id="rgbdns-frag-114e43800a7f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-114e43800a7f: fn rfc9619_standard_queries_require_exactly_one_question

```rust
fn rfc9619_standard_queries_require_exactly_one_question() {
    let mut empty = query("example", RecordType::A);
    empty.questions.clear();
    let answer = response(&empty);
    assert_eq!(rcode(&answer), 1);
    assert!(answer.questions.is_empty());

    let mut multiple = query("example", RecordType::A);
    multiple.questions.push(multiple.questions[0].clone());
    let answer = response(&multiple);
    assert_eq!(rcode(&answer), 1);
    assert!(answer.questions.is_empty());
}

#[test]
```
