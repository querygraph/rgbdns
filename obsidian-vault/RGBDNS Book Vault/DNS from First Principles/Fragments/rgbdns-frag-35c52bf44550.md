---
type: "code-fragment"
fragment_id: "rgbdns-frag-35c52bf44550"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "WireRecord"
kind: "struct"
start_line: 220
end_line: 227
---

# WireRecord

- Fragment ID: `rgbdns-frag-35c52bf44550`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 220-227
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-35c52bf44550", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 220, "endLine": 227}
```

## Excerpt

<span id="rgbdns-frag-35c52bf44550" class="rgbdns-fragment-target"></span>
### rgbdns-frag-35c52bf44550: struct WireRecord

```rust
struct WireRecord {
    name: Name,
    typ: u16,
    class: u16,
    ttl: u32,
    rdata: Vec<u8>,
}

```
