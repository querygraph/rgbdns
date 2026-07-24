---
type: "code-fragment"
fragment_id: "rgbdns-frag-e4b1690ff7a3"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc8906_unknown_opcode_is_notimp_even_when_its_body_is_unknown"
kind: "fn"
start_line: 38
end_line: 51
---

# rfc8906_unknown_opcode_is_notimp_even_when_its_body_is_unknown

- Fragment ID: `rgbdns-frag-e4b1690ff7a3`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 38-51
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-e4b1690ff7a3", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-e4b1690ff7a3: fn rfc8906_unknown_opcode_is_notimp_even_when_its_body_is_unknown", "sourcePath": "tests/rfc_conformance.rs", "startLine": 38, "endLine": 51}
```

## Excerpt

<span id="rgbdns-frag-e4b1690ff7a3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e4b1690ff7a3: fn rfc8906_unknown_opcode_is_notimp_even_when_its_body_is_unknown

```rust
fn rfc8906_unknown_opcode_is_notimp_even_when_its_body_is_unknown() {
    // An unknown opcode may define a different body layout. Only the header is
    // safe to interpret, so a truncated standard-question body must not turn
    // NOTIMP into FORMERR.
    let wire = [
        0x4a, 0x6f, 0x38, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let response = rgbdns::server::respond(&zone(), &wire, 4096).unwrap();
    let response = Message::decode(&response).unwrap();
    assert_eq!(rcode(&response), 4);
    assert_eq!(response.id, ID);
}

#[test]
```
