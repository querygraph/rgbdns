---
type: "code-fragment"
fragment_id: "rgbdns-frag-fc5933f616d0"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "wire_len"
kind: "fn"
start_line: 90
end_line: 92
---

# wire_len

- Fragment ID: `rgbdns-frag-fc5933f616d0`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 90-92
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-fc5933f616d0", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-fc5933f616d0: fn wire_len", "sourcePath": "src/name.rs", "startLine": 90, "endLine": 92}
```

## Excerpt

<span id="rgbdns-frag-fc5933f616d0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-fc5933f616d0: fn wire_len

```rust
    pub(crate) fn wire_len(&self) -> usize {
        1 + self.0.iter().map(|l| l.len() + 1).sum::<usize>()
    }
```
