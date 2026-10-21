---
type: "code-fragment"
fragment_id: "rgbdns-frag-89a40ae21437"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "Writer"
kind: "struct"
start_line: 621
end_line: 621
---

# Writer

- Fragment ID: `rgbdns-frag-89a40ae21437`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 621-621
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-89a40ae21437", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 621, "endLine": 621}
```

## Excerpt

<span id="rgbdns-frag-89a40ae21437" class="rgbdns-fragment-target"></span>
### rgbdns-frag-89a40ae21437: struct Writer

```rust
struct Writer(Vec<u8>, HashMap<Name, u16>, Option<(Name, u16)>);
```
