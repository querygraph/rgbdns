---
type: "code-fragment"
fragment_id: "rgbdns-frag-b1c09b3c9b5b"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "is_update"
kind: "fn"
start_line: 306
end_line: 311
---

# is_update

- Fragment ID: `rgbdns-frag-b1c09b3c9b5b`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 306-311
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b1c09b3c9b5b", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 306, "endLine": 311}
```

## Excerpt

<span id="rgbdns-frag-b1c09b3c9b5b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b1c09b3c9b5b: fn is_update

```rust
    pub fn is_update(wire: &[u8]) -> bool {
        wire.len() >= 4
            && u16::from_be_bytes([wire[2], wire[3]]) & 0x7800 == OPCODE_UPDATE
            && wire[2] & 0x80 == 0
    }

```
