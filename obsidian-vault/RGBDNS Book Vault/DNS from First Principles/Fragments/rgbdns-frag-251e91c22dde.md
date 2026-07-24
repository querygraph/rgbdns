---
type: "code-fragment"
fragment_id: "rgbdns-frag-251e91c22dde"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "from_labels"
kind: "fn"
start_line: 86
end_line: 89
---

# from_labels

- Fragment ID: `rgbdns-frag-251e91c22dde`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 86-89
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-251e91c22dde", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-251e91c22dde: fn from_labels", "sourcePath": "src/name.rs", "startLine": 86, "endLine": 89}
```

## Excerpt

<span id="rgbdns-frag-251e91c22dde" class="rgbdns-fragment-target"></span>
### rgbdns-frag-251e91c22dde: fn from_labels

```rust
    pub(crate) fn from_labels(labels: Vec<Vec<u8>>) -> Result<Self> {
        validate(&labels)?;
        Ok(Self(labels))
    }
```
