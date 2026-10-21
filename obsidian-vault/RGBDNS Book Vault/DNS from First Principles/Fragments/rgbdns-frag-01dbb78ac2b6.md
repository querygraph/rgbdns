---
type: "code-fragment"
fragment_id: "rgbdns-frag-01dbb78ac2b6"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "u16"
kind: "fn"
start_line: 626
end_line: 628
---

# u16

- Fragment ID: `rgbdns-frag-01dbb78ac2b6`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 626-628
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-01dbb78ac2b6", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 626, "endLine": 628}
```

## Excerpt

<span id="rgbdns-frag-01dbb78ac2b6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-01dbb78ac2b6: fn u16

```rust
    fn u16(&mut self, n: u16) {
        self.0.extend(n.to_be_bytes())
    }
```
