---
type: "code-fragment"
fragment_id: "rgbdns-frag-2fea2a3d9bbd"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "CheckStatus"
kind: "struct"
start_line: 42
end_line: 49
---

# CheckStatus

- Fragment ID: `rgbdns-frag-2fea2a3d9bbd`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 42-49
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-2fea2a3d9bbd", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 42, "endLine": 49}
```

## Excerpt

<span id="rgbdns-frag-2fea2a3d9bbd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2fea2a3d9bbd: struct CheckStatus

```rust
pub struct CheckStatus {
    pub zone: Name,
    pub serial: u32,
    pub key_tag: u16,
    pub earliest_expiration: u32,
    pub remaining: u64,
}

```
