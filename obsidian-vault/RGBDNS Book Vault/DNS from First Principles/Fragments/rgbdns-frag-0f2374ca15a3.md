---
type: "code-fragment"
fragment_id: "rgbdns-frag-0f2374ca15a3"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "u16"
kind: "fn"
start_line: 470
end_line: 472
---

# u16

- Fragment ID: `rgbdns-frag-0f2374ca15a3`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 470-472
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0f2374ca15a3", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-0f2374ca15a3: fn u16", "sourcePath": "src/packet.rs", "startLine": 470, "endLine": 472}
```

## Excerpt

<span id="rgbdns-frag-0f2374ca15a3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0f2374ca15a3: fn u16

```rust
    fn u16(&mut self, n: u16) {
        self.0.extend(n.to_be_bytes())
    }
```
