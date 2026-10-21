---
type: "code-fragment"
fragment_id: "rgbdns-frag-9dacc1b8570d"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "u32"
kind: "fn"
start_line: 920
end_line: 927
---

# u32

- Fragment ID: `rgbdns-frag-9dacc1b8570d`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 920-927
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9dacc1b8570d", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 920, "endLine": 927}
```

## Excerpt

<span id="rgbdns-frag-9dacc1b8570d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9dacc1b8570d: fn u32

```rust
    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_be_bytes([
            self.u8()?,
            self.u8()?,
            self.u8()?,
            self.u8()?,
        ]))
    }
```
