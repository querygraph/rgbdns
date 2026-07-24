---
type: "code-fragment"
fragment_id: "rgbdns-frag-6f81d0442a73"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "u32"
kind: "fn"
start_line: 215
end_line: 222
---

# u32

- Fragment ID: `rgbdns-frag-6f81d0442a73`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 215-222
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-6f81d0442a73", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-6f81d0442a73: fn u32", "sourcePath": "src/packet.rs", "startLine": 215, "endLine": 222}
```

## Excerpt

<span id="rgbdns-frag-6f81d0442a73" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6f81d0442a73: fn u32

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
