---
type: "code-fragment"
fragment_id: "rgbdns-frag-23b0f16cdcee"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "AdminAction"
kind: "enum"
start_line: 102
end_line: 109
---

# AdminAction

- Fragment ID: `rgbdns-frag-23b0f16cdcee`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 102-109
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-23b0f16cdcee", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 102, "endLine": 109}
```

## Excerpt

<span id="rgbdns-frag-23b0f16cdcee" class="rgbdns-fragment-target"></span>
### rgbdns-frag-23b0f16cdcee: enum AdminAction

```rust
pub enum AdminAction<'a> {
    Present(&'a [u8]),
    Cleanup(&'a [u8]),
    Clear,
}

/// Sends one authenticated administrative mutation through the daemon's RFC
/// 2136 interface, so the durable and live states change in one transaction.
```
