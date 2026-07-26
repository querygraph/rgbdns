---
type: "code-fragment"
fragment_id: "rgbdns-frag-ff4f3e36f844"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "negative_soa"
kind: "fn"
start_line: 220
end_line: 226
---

# negative_soa

- Fragment ID: `rgbdns-frag-ff4f3e36f844`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 220-226
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ff4f3e36f844", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-ff4f3e36f844: fn negative_soa", "sourcePath": "src/server.rs", "startLine": 220, "endLine": 226}
```

## Excerpt

<span id="rgbdns-frag-ff4f3e36f844" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ff4f3e36f844: fn negative_soa

```rust
fn negative_soa(mut record: crate::Record) -> crate::Record {
    if let crate::RData::Soa { minimum, .. } = &record.data {
        record.ttl = record.ttl.min(*minimum);
    }
    record
}

```
