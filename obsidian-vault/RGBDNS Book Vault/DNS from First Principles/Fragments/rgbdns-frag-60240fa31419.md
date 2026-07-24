---
type: "code-fragment"
fragment_id: "rgbdns-frag-60240fa31419"
source_path: "tests/wire_security.rs"
code_note: "DNS from First Principles/Code/tests/wire_security.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc1035_maximum_name_is_accepted_and_one_octet_more_is_rejected"
kind: "fn"
start_line: 12
end_line: 33
---

# rfc1035_maximum_name_is_accepted_and_one_octet_more_is_rejected

- Fragment ID: `rgbdns-frag-60240fa31419`
- Source file: [[DNS from First Principles/Code/tests/wire_security.rs.source|tests/wire_security.rs]]
- Lines: 12-33
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-60240fa31419", "codeNote": "DNS from First Principles/Code/tests/wire_security.rs.source", "heading": "rgbdns-frag-60240fa31419: fn rfc1035_maximum_name_is_accepted_and_one_octet_more_is_rejected", "sourcePath": "tests/wire_security.rs", "startLine": 12, "endLine": 33}
```

## Excerpt

<span id="rgbdns-frag-60240fa31419" class="rgbdns-fragment-target"></span>
### rgbdns-frag-60240fa31419: fn rfc1035_maximum_name_is_accepted_and_one_octet_more_is_rejected

```rust
fn rfc1035_maximum_name_is_accepted_and_one_octet_more_is_rejected() {
    let maximum = [
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(61),
    ]
    .join(".");
    let name: Name = maximum.parse().unwrap();
    assert_eq!(name.to_string().len(), maximum.len() + 1);

    let too_long = [
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(62),
    ]
    .join(".");
    assert!(too_long.parse::<Name>().is_err());
}

#[test]
```
