---
type: "code-fragment"
fragment_id: "rgbdns-frag-8aa07134a9d6"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "Policy"
kind: "struct"
start_line: 36
end_line: 43
---

# Policy

- Fragment ID: `rgbdns-frag-8aa07134a9d6`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 36-43
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-8aa07134a9d6", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 36, "endLine": 43}
```

## Excerpt

<span id="rgbdns-frag-8aa07134a9d6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8aa07134a9d6: struct Policy

```rust
struct Policy {
    key_name: Name,
    secret: Vec<u8>,
    zone: Name,
    ttl: u32,
}

#[derive(Clone)]
```
