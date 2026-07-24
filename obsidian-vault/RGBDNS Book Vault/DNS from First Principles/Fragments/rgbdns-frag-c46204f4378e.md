---
type: "code-fragment"
fragment_id: "rgbdns-frag-c46204f4378e"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "labels"
kind: "fn"
start_line: 69
end_line: 71
---

# labels

- Fragment ID: `rgbdns-frag-c46204f4378e`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 69-71
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c46204f4378e", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-c46204f4378e: fn labels", "sourcePath": "src/name.rs", "startLine": 69, "endLine": 71}
```

## Excerpt

<span id="rgbdns-frag-c46204f4378e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c46204f4378e: fn labels

```rust
    pub fn labels(&self) -> impl Iterator<Item = &[u8]> {
        self.0.iter().map(Vec::as_slice)
    }
```
