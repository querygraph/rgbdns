---
type: "code-fragment"
fragment_id: "rgbdns-frag-cc626067a0e6"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "Message"
kind: "struct"
start_line: 343
end_line: 351
---

# Message

- Fragment ID: `rgbdns-frag-cc626067a0e6`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 343-351
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-cc626067a0e6", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 343, "endLine": 351}
```

## Excerpt

<span id="rgbdns-frag-cc626067a0e6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-cc626067a0e6: struct Message

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
