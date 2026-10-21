---
type: "code-fragment"
fragment_id: "rgbdns-frag-ff2edfafbe50"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "Overlay"
kind: "type"
start_line: 33
end_line: 35
---

# Overlay

- Fragment ID: `rgbdns-frag-ff2edfafbe50`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 33-35
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ff2edfafbe50", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 33, "endLine": 35}
```

## Excerpt

<span id="rgbdns-frag-ff2edfafbe50" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ff2edfafbe50: type Overlay

```rust
type Overlay = BTreeMap<Name, BTreeSet<Vec<u8>>>;

#[derive(Clone)]
```
