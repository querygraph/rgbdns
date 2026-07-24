---
type: "code-fragment"
fragment_id: "rgbdns-frag-4dd62ba91963"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "u32"
kind: "fn"
start_line: 473
end_line: 475
---

# u32

- Fragment ID: `rgbdns-frag-4dd62ba91963`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 473-475
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4dd62ba91963", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-4dd62ba91963: fn u32", "sourcePath": "src/packet.rs", "startLine": 473, "endLine": 475}
```

## Excerpt

<span id="rgbdns-frag-4dd62ba91963" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4dd62ba91963: fn u32

```rust
    fn u32(&mut self, n: u32) {
        self.0.extend(n.to_be_bytes())
    }
```
