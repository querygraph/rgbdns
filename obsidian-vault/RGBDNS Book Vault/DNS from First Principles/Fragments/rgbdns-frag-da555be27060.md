---
type: "code-fragment"
fragment_id: "rgbdns-frag-da555be27060"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc2308_negative_soa_ttl_is_the_minimum_of_soa_ttl_and_minimum"
kind: "fn"
start_line: 113
end_line: 124
---

# rfc2308_negative_soa_ttl_is_the_minimum_of_soa_ttl_and_minimum

- Fragment ID: `rgbdns-frag-da555be27060`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 113-124
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-da555be27060", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-da555be27060: fn rfc2308_negative_soa_ttl_is_the_minimum_of_soa_ttl_and_minimum", "sourcePath": "tests/rfc_conformance.rs", "startLine": 113, "endLine": 124}
```

## Excerpt

<span id="rgbdns-frag-da555be27060" class="rgbdns-fragment-target"></span>
### rgbdns-frag-da555be27060: fn rfc2308_negative_soa_ttl_is_the_minimum_of_soa_ttl_and_minimum

```rust
fn rfc2308_negative_soa_ttl_is_the_minimum_of_soa_ttl_and_minimum() {
    let zone =
        rgbdns::zone::Zone::parse("Zexample:ns.example:hostmaster.example:1:2:3:4:60:3600\n")
            .unwrap();
    let request = query("missing.example", RecordType::A);
    let wire = rgbdns::server::respond(&zone, &request.encode().unwrap(), 4096).unwrap();
    let answer = Message::decode(&wire).unwrap();
    assert_eq!(rcode(&answer), 3);
    assert_eq!(answer.authorities[0].ttl, 60);
}

#[test]
```
