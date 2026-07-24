---
type: "code-fragment"
fragment_id: "rgbdns-frag-c8980ffb1624"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "u16"
kind: "fn"
start_line: 212
end_line: 214
---

# u16

- Fragment ID: `rgbdns-frag-c8980ffb1624`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 212-214
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c8980ffb1624", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-c8980ffb1624: fn u16", "sourcePath": "src/packet.rs", "startLine": 212, "endLine": 214}
```

## Excerpt

<span id="rgbdns-frag-c8980ffb1624" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c8980ffb1624: fn u16

```rust
    fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_be_bytes([self.u8()?, self.u8()?]))
    }
```
