---
type: "code-fragment"
fragment_id: "rgbdns-frag-ca2394e2c026"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc4592_closest_encloser_blocks_a_higher_wildcard"
kind: "fn"
start_line: 250
end_line: 265
---

# rfc4592_closest_encloser_blocks_a_higher_wildcard

- Fragment ID: `rgbdns-frag-ca2394e2c026`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 250-265
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-ca2394e2c026", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-ca2394e2c026: fn rfc4592_closest_encloser_blocks_a_higher_wildcard", "sourcePath": "tests/rfc_conformance.rs", "startLine": 250, "endLine": 265}
```

## Excerpt

<span id="rgbdns-frag-ca2394e2c026" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ca2394e2c026: fn rfc4592_closest_encloser_blocks_a_higher_wildcard

```rust
fn rfc4592_closest_encloser_blocks_a_higher_wildcard() {
    let zone = rgbdns::zone::Zone::parse(
        ".example::ns.example\n\
         +*.example:192.0.2.1\n\
         +node.branch.example:192.0.2.2\n",
    )
    .unwrap();
    let request = query("missing.branch.example", RecordType::A);
    let answer =
        Message::decode(&rgbdns::server::respond(&zone, &request.encode().unwrap(), 4096).unwrap())
            .unwrap();
    assert_eq!(rcode(&answer), 3);
    assert!(answer.answers.is_empty());
}

#[test]
```
