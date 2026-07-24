---
type: "code-fragment"
fragment_id: "rgbdns-frag-7ff5ba1eb6d3"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "to_wire"
kind: "fn"
start_line: 96
end_line: 104
---

# to_wire

- Fragment ID: `rgbdns-frag-7ff5ba1eb6d3`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 96-104
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7ff5ba1eb6d3", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-7ff5ba1eb6d3: fn to_wire", "sourcePath": "src/name.rs", "startLine": 96, "endLine": 104}
```

## Excerpt

<span id="rgbdns-frag-7ff5ba1eb6d3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7ff5ba1eb6d3: fn to_wire

```rust
    pub(crate) fn to_wire(&self) -> Vec<u8> {
        let mut wire = Vec::with_capacity(self.wire_len());
        for label in &self.0 {
            wire.push(label.len() as u8);
            wire.extend(label);
        }
        wire.push(0);
        wire
    }
```
