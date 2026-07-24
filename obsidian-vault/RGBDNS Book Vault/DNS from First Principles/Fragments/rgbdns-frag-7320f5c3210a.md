---
type: "code-fragment"
fragment_id: "rgbdns-frag-7320f5c3210a"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "negative_soa"
kind: "fn"
start_line: 179
end_line: 185
---

# negative_soa

- Fragment ID: `rgbdns-frag-7320f5c3210a`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 179-185
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7320f5c3210a", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-7320f5c3210a: fn negative_soa", "sourcePath": "src/server.rs", "startLine": 179, "endLine": 185}
```

## Excerpt

<span id="rgbdns-frag-7320f5c3210a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7320f5c3210a: fn negative_soa

```rust
fn negative_soa(mut record: crate::Record) -> crate::Record {
    if let crate::RData::Soa { minimum, .. } = &record.data {
        record.ttl = record.ttl.min(*minimum);
    }
    record
}

```
