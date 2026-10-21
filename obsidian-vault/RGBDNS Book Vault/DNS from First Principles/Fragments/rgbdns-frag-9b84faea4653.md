---
type: "code-fragment"
fragment_id: "rgbdns-frag-9b84faea4653"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "negative_soa"
kind: "fn"
start_line: 420
end_line: 426
---

# negative_soa

- Fragment ID: `rgbdns-frag-9b84faea4653`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 420-426
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9b84faea4653", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 420, "endLine": 426}
```

## Excerpt

<span id="rgbdns-frag-9b84faea4653" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9b84faea4653: fn negative_soa

```rust
fn negative_soa(mut record: crate::Record) -> crate::Record {
    if let crate::RData::Soa { minimum, .. } = &record.data {
        record.ttl = record.ttl.min(*minimum);
    }
    record
}

```
