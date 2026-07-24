---
type: "code-fragment"
fragment_id: "rgbdns-frag-dda2855f47d7"
source_path: "src/conf.rs"
code_note: "DNS from First Principles/Code/src/conf.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "Service"
kind: "enum"
start_line: 11
end_line: 19
---

# Service

- Fragment ID: `rgbdns-frag-dda2855f47d7`
- Source file: [[DNS from First Principles/Code/src/conf.rs.source|src/conf.rs]]
- Lines: 11-19
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-dda2855f47d7", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-dda2855f47d7: enum Service", "sourcePath": "src/conf.rs", "startLine": 11, "endLine": 19}
```

## Excerpt

<span id="rgbdns-frag-dda2855f47d7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-dda2855f47d7: enum Service

```rust
pub enum Service {
    Tinydns,
    Dnscache,
    Rbldns,
    Pickdns,
    Walldns,
    Axfrdns,
}

```
