---
type: "code-fragment"
fragment_id: "rgbdns-frag-cf52568bbe9c"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "suffix"
kind: "fn"
start_line: 93
end_line: 95
---

# suffix

- Fragment ID: `rgbdns-frag-cf52568bbe9c`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 93-95
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-cf52568bbe9c", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-cf52568bbe9c: fn suffix", "sourcePath": "src/name.rs", "startLine": 93, "endLine": 95}
```

## Excerpt

<span id="rgbdns-frag-cf52568bbe9c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-cf52568bbe9c: fn suffix

```rust
    pub(crate) fn suffix(&self, first_label: usize) -> Self {
        Self(self.0[first_label..].to_vec())
    }
```
