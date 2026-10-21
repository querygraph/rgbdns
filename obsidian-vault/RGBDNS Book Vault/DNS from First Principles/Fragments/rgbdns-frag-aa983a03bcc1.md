---
type: "code-fragment"
fragment_id: "rgbdns-frag-aa983a03bcc1"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "to_wire"
kind: "fn"
start_line: 108
end_line: 116
---

# to_wire

- Fragment ID: `rgbdns-frag-aa983a03bcc1`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 108-116
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-aa983a03bcc1", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "sourcePath": "src/name.rs", "startLine": 108, "endLine": 116}
```

## Excerpt

<span id="rgbdns-frag-aa983a03bcc1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-aa983a03bcc1: fn to_wire

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
