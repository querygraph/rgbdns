---
type: "code-fragment"
fragment_id: "rgbdns-frag-3736c8d8aac5"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "suffix"
kind: "fn"
start_line: 105
end_line: 107
---

# suffix

- Fragment ID: `rgbdns-frag-3736c8d8aac5`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 105-107
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3736c8d8aac5", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "sourcePath": "src/name.rs", "startLine": 105, "endLine": 107}
```

## Excerpt

<span id="rgbdns-frag-3736c8d8aac5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3736c8d8aac5: fn suffix

```rust
    pub(crate) fn suffix(&self, first_label: usize) -> Self {
        Self(self.0[first_label..].to_vec())
    }
```
