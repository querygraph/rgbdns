---
type: "code-fragment"
fragment_id: "rgbdns-frag-04e910bc5479"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc6891_duplicate_or_misplaced_opt_is_formerr"
kind: "fn"
start_line: 188
end_line: 203
---

# rfc6891_duplicate_or_misplaced_opt_is_formerr

- Fragment ID: `rgbdns-frag-04e910bc5479`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 188-203
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-04e910bc5479", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-04e910bc5479: fn rfc6891_duplicate_or_misplaced_opt_is_formerr", "sourcePath": "tests/rfc_conformance.rs", "startLine": 188, "endLine": 203}
```

## Excerpt

<span id="rgbdns-frag-04e910bc5479" class="rgbdns-fragment-target"></span>
### rgbdns-frag-04e910bc5479: fn rfc6891_duplicate_or_misplaced_opt_is_formerr

```rust
fn rfc6891_duplicate_or_misplaced_opt_is_formerr() {
    let mut duplicate = query("www.example", RecordType::A);
    duplicate.additionals.push(opt(1232, 0, 0, Vec::new()));
    duplicate.additionals.push(opt(1232, 0, 0, Vec::new()));
    assert_eq!(rcode(&response(&duplicate)), 1);

    let mut answer_opt = query("www.example", RecordType::A);
    answer_opt.answers.push(opt(1232, 0, 0, Vec::new()));
    assert_eq!(rcode(&response(&answer_opt)), 1);

    let mut authority_opt = query("www.example", RecordType::A);
    authority_opt.authorities.push(opt(1232, 0, 0, Vec::new()));
    assert_eq!(rcode(&response(&authority_opt)), 1);
}

#[test]
```
