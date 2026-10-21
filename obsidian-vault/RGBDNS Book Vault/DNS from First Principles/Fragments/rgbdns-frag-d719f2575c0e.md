---
type: "code-fragment"
fragment_id: "rgbdns-frag-d719f2575c0e"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "u16"
kind: "fn"
start_line: 366
end_line: 368
---

# u16

- Fragment ID: `rgbdns-frag-d719f2575c0e`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 366-368
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d719f2575c0e", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 366, "endLine": 368}
```

## Excerpt

<span id="rgbdns-frag-d719f2575c0e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d719f2575c0e: fn u16

```rust
    fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_be_bytes([self.u8()?, self.u8()?]))
    }
```
