---
type: "code-fragment"
fragment_id: "rgbdns-frag-89725a937c01"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc2181_rrset_ttls_are_equalized_and_duplicate_records_suppressed"
kind: "fn"
start_line: 280
end_line: 295
---

# rfc2181_rrset_ttls_are_equalized_and_duplicate_records_suppressed

- Fragment ID: `rgbdns-frag-89725a937c01`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 280-295
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-89725a937c01", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-89725a937c01: fn rfc2181_rrset_ttls_are_equalized_and_duplicate_records_suppressed", "sourcePath": "tests/rfc_conformance.rs", "startLine": 280, "endLine": 295}
```

## Excerpt

<span id="rgbdns-frag-89725a937c01" class="rgbdns-fragment-target"></span>
### rgbdns-frag-89725a937c01: fn rfc2181_rrset_ttls_are_equalized_and_duplicate_records_suppressed

```rust
fn rfc2181_rrset_ttls_are_equalized_and_duplicate_records_suppressed() {
    let zone = rgbdns::zone::Zone::parse(
        ".example::ns.example\n\
         +multi.example:192.0.2.1:600\n\
         +multi.example:192.0.2.2:300\n\
         +multi.example:192.0.2.1:900\n",
    )
    .unwrap();
    let request = query("multi.example", RecordType::A);
    let wire = rgbdns::server::respond(&zone, &request.encode().unwrap(), 4096).unwrap();
    let response = Message::decode(&wire).unwrap();
    assert_eq!(response.answers.len(), 2);
    assert!(response.answers.iter().all(|record| record.ttl == 300));
}

#[test]
```
