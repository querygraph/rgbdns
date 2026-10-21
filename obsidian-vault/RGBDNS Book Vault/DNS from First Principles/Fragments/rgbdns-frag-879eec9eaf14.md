---
type: "code-fragment"
fragment_id: "rgbdns-frag-879eec9eaf14"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "is_dnssec_type"
kind: "fn"
start_line: 327
end_line: 338
---

# is_dnssec_type

- Fragment ID: `rgbdns-frag-879eec9eaf14`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 327-338
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-879eec9eaf14", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 327, "endLine": 338}
```

## Excerpt

<span id="rgbdns-frag-879eec9eaf14" class="rgbdns-fragment-target"></span>
### rgbdns-frag-879eec9eaf14: fn is_dnssec_type

```rust
fn is_dnssec_type(record_type: crate::RecordType) -> bool {
    matches!(
        record_type,
        crate::RecordType::Ds
            | crate::RecordType::Rrsig
            | crate::RecordType::Nsec
            | crate::RecordType::Dnskey
            | crate::RecordType::Unknown(50)
            | crate::RecordType::Unknown(51)
    )
}

```
