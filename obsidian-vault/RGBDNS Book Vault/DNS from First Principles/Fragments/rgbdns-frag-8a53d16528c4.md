---
type: "code-fragment"
fragment_id: "rgbdns-frag-8a53d16528c4"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "Record"
kind: "struct"
start_line: 124
end_line: 128
---

# Record

- Fragment ID: `rgbdns-frag-8a53d16528c4`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 124-128
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-8a53d16528c4", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-8a53d16528c4: struct Record", "sourcePath": "src/packet.rs", "startLine": 124, "endLine": 128}
```

## Excerpt

<span id="rgbdns-frag-8a53d16528c4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8a53d16528c4: struct Record

```rust
pub struct Record {
    pub name: Name,
    pub ttl: u32,
    pub data: RData,
}
```
