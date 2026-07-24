---
type: "code-fragment"
fragment_id: "rgbdns-frag-f8e2f24b275d"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "Message"
kind: "struct"
start_line: 189
end_line: 197
---

# Message

- Fragment ID: `rgbdns-frag-f8e2f24b275d`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 189-197
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f8e2f24b275d", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-f8e2f24b275d: struct Message", "sourcePath": "src/packet.rs", "startLine": 189, "endLine": 197}
```

## Excerpt

<span id="rgbdns-frag-f8e2f24b275d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f8e2f24b275d: struct Message

```rust
pub struct Message {
    pub id: u16,
    pub flags: u16,
    pub questions: Vec<Question>,
    pub answers: Vec<Record>,
    pub authorities: Vec<Record>,
    pub additionals: Vec<Record>,
}

```
