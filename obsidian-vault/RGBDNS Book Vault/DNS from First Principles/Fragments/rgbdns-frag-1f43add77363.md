---
type: "code-fragment"
fragment_id: "rgbdns-frag-1f43add77363"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "Question"
kind: "struct"
start_line: 118
end_line: 123
---

# Question

- Fragment ID: `rgbdns-frag-1f43add77363`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 118-123
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1f43add77363", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-1f43add77363: struct Question", "sourcePath": "src/packet.rs", "startLine": 118, "endLine": 123}
```

## Excerpt

<span id="rgbdns-frag-1f43add77363" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1f43add77363: struct Question

```rust
pub struct Question {
    pub name: Name,
    pub qtype: RecordType,
    pub qclass: u16,
}
#[derive(Clone, Debug, Eq, PartialEq)]
```
