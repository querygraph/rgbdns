---
type: "code-fragment"
fragment_id: "rgbdns-frag-874de4715a3a"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "parent"
kind: "fn"
start_line: 75
end_line: 77
---

# parent

- Fragment ID: `rgbdns-frag-874de4715a3a`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 75-77
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-874de4715a3a", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-874de4715a3a: fn parent", "sourcePath": "src/name.rs", "startLine": 75, "endLine": 77}
```

## Excerpt

<span id="rgbdns-frag-874de4715a3a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-874de4715a3a: fn parent

```rust
    pub fn parent(&self) -> Option<Self> {
        (!self.0.is_empty()).then(|| Self(self.0[1..].to_vec()))
    }
```
