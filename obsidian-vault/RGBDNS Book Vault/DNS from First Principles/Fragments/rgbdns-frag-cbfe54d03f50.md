---
type: "code-fragment"
fragment_id: "rgbdns-frag-cbfe54d03f50"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "u32"
kind: "fn"
start_line: 629
end_line: 631
---

# u32

- Fragment ID: `rgbdns-frag-cbfe54d03f50`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 629-631
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-cbfe54d03f50", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 629, "endLine": 631}
```

## Excerpt

<span id="rgbdns-frag-cbfe54d03f50" class="rgbdns-fragment-target"></span>
### rgbdns-frag-cbfe54d03f50: fn u32

```rust
    fn u32(&mut self, n: u32) {
        self.0.extend(n.to_be_bytes())
    }
```
