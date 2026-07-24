---
type: "code-fragment"
fragment_id: "rgbdns-frag-7c89404221eb"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "is_root"
kind: "fn"
start_line: 72
end_line: 74
---

# is_root

- Fragment ID: `rgbdns-frag-7c89404221eb`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 72-74
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7c89404221eb", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-7c89404221eb: fn is_root", "sourcePath": "src/name.rs", "startLine": 72, "endLine": 74}
```

## Excerpt

<span id="rgbdns-frag-7c89404221eb" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7c89404221eb: fn is_root

```rust
    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }
```
