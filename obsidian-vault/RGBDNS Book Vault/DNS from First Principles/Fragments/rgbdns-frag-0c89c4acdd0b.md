---
type: "code-fragment"
fragment_id: "rgbdns-frag-0c89c4acdd0b"
source_path: "src/tinydns_edit.rs"
code_note: "DNS from First Principles/Code/src/tinydns_edit.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "Address"
kind: "enum"
start_line: 38
end_line: 42
---

# Address

- Fragment ID: `rgbdns-frag-0c89c4acdd0b`
- Source file: [[DNS from First Principles/Code/src/tinydns_edit.rs.source|src/tinydns_edit.rs]]
- Lines: 38-42
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0c89c4acdd0b", "codeNote": "DNS from First Principles/Code/src/tinydns_edit.rs.source", "heading": "rgbdns-frag-0c89c4acdd0b: enum Address", "sourcePath": "src/tinydns_edit.rs", "startLine": 38, "endLine": 42}
```

## Excerpt

<span id="rgbdns-frag-0c89c4acdd0b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0c89c4acdd0b: enum Address

```rust
pub enum Address {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

```
