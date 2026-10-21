---
type: "code-fragment"
fragment_id: "rgbdns-frag-9a4884d9edd2"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "from_labels"
kind: "fn"
start_line: 98
end_line: 101
---

# from_labels

- Fragment ID: `rgbdns-frag-9a4884d9edd2`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 98-101
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9a4884d9edd2", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "sourcePath": "src/name.rs", "startLine": 98, "endLine": 101}
```

## Excerpt

<span id="rgbdns-frag-9a4884d9edd2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9a4884d9edd2: fn from_labels

```rust
    pub(crate) fn from_labels(labels: Vec<Vec<u8>>) -> Result<Self> {
        validate(&labels)?;
        Ok(Self(labels))
    }
```
