---
type: "code-fragment"
fragment_id: "rgbdns-frag-0112dc40f9af"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "Reader"
kind: "struct"
start_line: 352
end_line: 356
---

# Reader

- Fragment ID: `rgbdns-frag-0112dc40f9af`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 352-356
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0112dc40f9af", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 352, "endLine": 356}
```

## Excerpt

<span id="rgbdns-frag-0112dc40f9af" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0112dc40f9af: struct Reader

```rust
struct Reader<'a> {
    b: &'a [u8],
    p: usize,
    name_offsets: Vec<bool>,
}
```
