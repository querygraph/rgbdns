---
type: "code-fragment"
fragment_id: "rgbdns-frag-bb7e008e7e8c"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc2308_nodata_and_nxdomain_are_distinct_and_include_soa"
kind: "fn"
start_line: 100
end_line: 112
---

# rfc2308_nodata_and_nxdomain_are_distinct_and_include_soa

- Fragment ID: `rgbdns-frag-bb7e008e7e8c`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 100-112
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-bb7e008e7e8c", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-bb7e008e7e8c: fn rfc2308_nodata_and_nxdomain_are_distinct_and_include_soa", "sourcePath": "tests/rfc_conformance.rs", "startLine": 100, "endLine": 112}
```

## Excerpt

<span id="rgbdns-frag-bb7e008e7e8c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bb7e008e7e8c: fn rfc2308_nodata_and_nxdomain_are_distinct_and_include_soa

```rust
fn rfc2308_nodata_and_nxdomain_are_distinct_and_include_soa() {
    let nodata = response(&query("www.example", RecordType::Mx));
    assert_eq!(rcode(&nodata), 0);
    assert!(nodata.answers.is_empty());
    assert_eq!(nodata.authorities[0].rr_type(), RecordType::Soa);

    let nxdomain = response(&query("missing.example", RecordType::A));
    assert_eq!(rcode(&nxdomain), 3);
    assert!(nxdomain.answers.is_empty());
    assert_eq!(nxdomain.authorities[0].rr_type(), RecordType::Soa);
}

#[test]
```
