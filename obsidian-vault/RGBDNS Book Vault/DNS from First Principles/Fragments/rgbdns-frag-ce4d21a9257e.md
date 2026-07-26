---
type: "code-fragment"
fragment_id: "rgbdns-frag-ce4d21a9257e"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "Lookup"
kind: "enum"
start_line: 690
end_line: 699
---

# Lookup

- Fragment ID: `rgbdns-frag-ce4d21a9257e`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 690-699
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ce4d21a9257e", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-ce4d21a9257e: enum Lookup", "sourcePath": "src/zone.rs", "startLine": 690, "endLine": 699}
```

## Excerpt

<span id="rgbdns-frag-ce4d21a9257e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ce4d21a9257e: enum Lookup

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
