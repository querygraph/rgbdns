---
type: "code-fragment"
fragment_id: "rgbdns-frag-fdf99e5a5810"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "u8"
kind: "fn"
start_line: 358
end_line: 365
---

# u8

- Fragment ID: `rgbdns-frag-fdf99e5a5810`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 358-365
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-fdf99e5a5810", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 358, "endLine": 365}
```

## Excerpt

<span id="rgbdns-frag-fdf99e5a5810" class="rgbdns-fragment-target"></span>
### rgbdns-frag-fdf99e5a5810: fn u8

```rust
    fn u8(&mut self) -> Result<u8> {
        let x = *self
            .b
            .get(self.p)
            .ok_or(Error::Format("truncated packet"))?;
        self.p += 1;
        Ok(x)
    }
```
