---
type: "code-fragment"
fragment_id: "rgbdns-frag-4e7a403d313a"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "Reader"
kind: "struct"
start_line: 198
end_line: 202
---

# Reader

- Fragment ID: `rgbdns-frag-4e7a403d313a`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 198-202
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4e7a403d313a", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-4e7a403d313a: struct Reader", "sourcePath": "src/packet.rs", "startLine": 198, "endLine": 202}
```

## Excerpt

<span id="rgbdns-frag-4e7a403d313a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4e7a403d313a: struct Reader

```rust
struct Reader<'a> {
    b: &'a [u8],
    p: usize,
    name_offsets: Vec<bool>,
}
```
