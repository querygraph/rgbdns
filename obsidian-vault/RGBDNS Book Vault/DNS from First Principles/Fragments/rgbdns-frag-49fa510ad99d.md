---
type: "code-fragment"
fragment_id: "rgbdns-frag-49fa510ad99d"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "u16"
kind: "fn"
start_line: 917
end_line: 919
---

# u16

- Fragment ID: `rgbdns-frag-49fa510ad99d`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 917-919
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-49fa510ad99d", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 917, "endLine": 919}
```

## Excerpt

<span id="rgbdns-frag-49fa510ad99d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-49fa510ad99d: fn u16

```rust
    fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_be_bytes([self.u8()?, self.u8()?]))
    }
```
