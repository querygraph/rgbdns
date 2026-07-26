---
type: "code-fragment"
fragment_id: "rgbdns-frag-ca380a5004ce"
source_path: "src/cdb.rs"
code_note: "DNS from First Principles/Code/src/cdb.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "le_u32"
kind: "fn"
start_line: 297
end_line: 301
---

# le_u32

- Fragment ID: `rgbdns-frag-ca380a5004ce`
- Source file: [[DNS from First Principles/Code/src/cdb.rs.source|src/cdb.rs]]
- Lines: 297-301
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ca380a5004ce", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-ca380a5004ce: fn le_u32", "sourcePath": "src/cdb.rs", "startLine": 297, "endLine": 301}
```

## Excerpt

<span id="rgbdns-frag-ca380a5004ce" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ca380a5004ce: fn le_u32

```rust
fn le_u32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

#[cfg(test)]
```
