---
type: "code-fragment"
fragment_id: "rgbdns-frag-b85d37b06c18"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "u8"
kind: "fn"
start_line: 623
end_line: 625
---

# u8

- Fragment ID: `rgbdns-frag-b85d37b06c18`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 623-625
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b85d37b06c18", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 623, "endLine": 625}
```

## Excerpt

<span id="rgbdns-frag-b85d37b06c18" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b85d37b06c18: fn u8

```rust
    fn u8(&mut self, n: u8) {
        self.0.push(n)
    }
```
