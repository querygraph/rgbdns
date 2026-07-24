---
type: "code-fragment"
fragment_id: "rgbdns-frag-63702d1be377"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc8906_badvers_survives_unknown_edns_flags_and_options"
kind: "fn"
start_line: 170
end_line: 187
---

# rfc8906_badvers_survives_unknown_edns_flags_and_options

- Fragment ID: `rgbdns-frag-63702d1be377`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 170-187
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-63702d1be377", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-63702d1be377: fn rfc8906_badvers_survives_unknown_edns_flags_and_options", "sourcePath": "tests/rfc_conformance.rs", "startLine": 170, "endLine": 187}
```

## Excerpt

<span id="rgbdns-frag-63702d1be377" class="rgbdns-fragment-target"></span>
### rgbdns-frag-63702d1be377: fn rfc8906_badvers_survives_unknown_edns_flags_and_options

```rust
fn rfc8906_badvers_survives_unknown_edns_flags_and_options() {
    let mut request = query("www.example", RecordType::A);
    request
        .additionals
        .push(opt(1232, 255, 0xffff, vec![0xfd, 0xe8, 0, 3, 1, 2, 3]));
    let response = response(&request);
    assert_eq!(extended_rcode(&response), 16);
    assert!(matches!(
        response.additionals.last().unwrap().data,
        RData::Opt {
            version: 0,
            flags: 0x8000,
            ..
        }
    ));
}

#[test]
```
