---
type: "code-fragment"
fragment_id: "rgbdns-frag-4aadcae32320"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "u8"
kind: "fn"
start_line: 467
end_line: 469
---

# u8

- Fragment ID: `rgbdns-frag-4aadcae32320`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 467-469
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4aadcae32320", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-4aadcae32320: fn u8", "sourcePath": "src/packet.rs", "startLine": 467, "endLine": 469}
```

## Excerpt

<span id="rgbdns-frag-4aadcae32320" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4aadcae32320: fn u8

```rust
    fn u8(&mut self, n: u8) {
        self.0.push(n)
    }
```
