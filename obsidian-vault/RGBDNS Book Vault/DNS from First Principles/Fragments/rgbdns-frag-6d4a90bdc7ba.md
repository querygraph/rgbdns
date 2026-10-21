---
type: "code-fragment"
fragment_id: "rgbdns-frag-6d4a90bdc7ba"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "u32"
kind: "fn"
start_line: 369
end_line: 376
---

# u32

- Fragment ID: `rgbdns-frag-6d4a90bdc7ba`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 369-376
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-6d4a90bdc7ba", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 369, "endLine": 376}
```

## Excerpt

<span id="rgbdns-frag-6d4a90bdc7ba" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6d4a90bdc7ba: fn u32

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
