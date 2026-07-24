---
type: "code-fragment"
fragment_id: "rgbdns-frag-98d21a984576"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc3597_unknown_rdata_roundtrips_losslessly"
kind: "fn"
start_line: 223
end_line: 239
---

# rfc3597_unknown_rdata_roundtrips_losslessly

- Fragment ID: `rgbdns-frag-98d21a984576`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 223-239
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-98d21a984576", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-98d21a984576: fn rfc3597_unknown_rdata_roundtrips_losslessly", "sourcePath": "tests/rfc_conformance.rs", "startLine": 223, "endLine": 239}
```

## Excerpt

<span id="rgbdns-frag-98d21a984576" class="rgbdns-fragment-target"></span>
### rgbdns-frag-98d21a984576: fn rfc3597_unknown_rdata_roundtrips_losslessly

```rust
fn rfc3597_unknown_rdata_roundtrips_losslessly() {
    let original = Message {
        id: ID,
        answers: vec![rgbdns::Record {
            name: "opaque.example".parse().unwrap(),
            ttl: 1234,
            data: RData::Opaque(RecordType::Unknown(65_000), vec![0, 1, 2, 0xff]),
        }],
        ..Message::default()
    };
    assert_eq!(
        Message::decode(&original.encode().unwrap()).unwrap(),
        original
    );
}

#[test]
```
