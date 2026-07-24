---
type: "code-fragment"
fragment_id: "rgbdns-frag-d6b7f8d36179"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "Lookup"
kind: "enum"
start_line: 620
end_line: 629
---

# Lookup

- Fragment ID: `rgbdns-frag-d6b7f8d36179`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 620-629
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d6b7f8d36179", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-d6b7f8d36179: enum Lookup", "sourcePath": "src/zone.rs", "startLine": 620, "endLine": 629}
```

## Excerpt

<span id="rgbdns-frag-d6b7f8d36179" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d6b7f8d36179: enum Lookup

```rust
pub enum Lookup {
    Answer(Vec<Record>),
    Referral {
        authorities: Vec<Record>,
        additionals: Vec<Record>,
    },
    NoData(Option<Record>),
    NxDomain(Option<Record>),
    Refused,
}
```
