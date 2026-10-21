---
type: "code-fragment"
fragment_id: "rgbdns-frag-183dcd88cb73"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "Lookup"
kind: "enum"
start_line: 958
end_line: 967
---

# Lookup

- Fragment ID: `rgbdns-frag-183dcd88cb73`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 958-967
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-183dcd88cb73", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 958, "endLine": 967}
```

## Excerpt

<span id="rgbdns-frag-183dcd88cb73" class="rgbdns-fragment-target"></span>
### rgbdns-frag-183dcd88cb73: enum Lookup

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
