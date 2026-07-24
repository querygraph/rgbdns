---
type: "code-fragment"
fragment_id: "rgbdns-frag-e1d3b03b1b53"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "u8"
kind: "fn"
start_line: 204
end_line: 211
---

# u8

- Fragment ID: `rgbdns-frag-e1d3b03b1b53`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 204-211
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e1d3b03b1b53", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-e1d3b03b1b53: fn u8", "sourcePath": "src/packet.rs", "startLine": 204, "endLine": 211}
```

## Excerpt

<span id="rgbdns-frag-e1d3b03b1b53" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e1d3b03b1b53: fn u8

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
