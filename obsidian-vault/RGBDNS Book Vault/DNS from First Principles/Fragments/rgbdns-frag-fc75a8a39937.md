---
type: "code-fragment"
fragment_id: "rgbdns-frag-fc75a8a39937"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "u8"
kind: "fn"
start_line: 909
end_line: 916
---

# u8

- Fragment ID: `rgbdns-frag-fc75a8a39937`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 909-916
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-fc75a8a39937", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 909, "endLine": 916}
```

## Excerpt

<span id="rgbdns-frag-fc75a8a39937" class="rgbdns-fragment-target"></span>
### rgbdns-frag-fc75a8a39937: fn u8

```rust
    fn u8(&mut self) -> Result<u8> {
        let value = *self
            .wire
            .get(self.position)
            .ok_or(Error::Format("truncated UPDATE"))?;
        self.position += 1;
        Ok(value)
    }
```
