---
type: "code-fragment"
fragment_id: "rgbdns-frag-d3d879f22833"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "MaterializationInput"
kind: "type"
start_line: 35
end_line: 36
---

# MaterializationInput

- Fragment ID: `rgbdns-frag-d3d879f22833`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 35-36
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d3d879f22833", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 35, "endLine": 36}
```

## Excerpt

<span id="rgbdns-frag-d3d879f22833" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d3d879f22833: type MaterializationInput

```rust
pub(crate) type MaterializationInput = (Vec<Record>, Vec<(Name, Aname)>);
/// The later of two SOA serials in RFC 1982 serial-number order.
```
