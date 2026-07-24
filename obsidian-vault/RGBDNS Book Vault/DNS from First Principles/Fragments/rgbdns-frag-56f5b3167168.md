---
type: "code-fragment"
fragment_id: "rgbdns-frag-56f5b3167168"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "Writer"
kind: "struct"
start_line: 465
end_line: 465
---

# Writer

- Fragment ID: `rgbdns-frag-56f5b3167168`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 465-465
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-56f5b3167168", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-56f5b3167168: struct Writer", "sourcePath": "src/packet.rs", "startLine": 465, "endLine": 465}
```

## Excerpt

<span id="rgbdns-frag-56f5b3167168" class="rgbdns-fragment-target"></span>
### rgbdns-frag-56f5b3167168: struct Writer

```rust
struct Writer(Vec<u8>, HashMap<Name, u16>, Option<(Name, u16)>);
```
