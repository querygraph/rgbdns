---
type: "code-fragment"
fragment_id: "rgbdns-frag-ea6bdb9d177f"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "wildcard"
kind: "fn"
start_line: 81
end_line: 85
---

# wildcard

- Fragment ID: `rgbdns-frag-ea6bdb9d177f`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 81-85
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ea6bdb9d177f", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-ea6bdb9d177f: fn wildcard", "sourcePath": "src/name.rs", "startLine": 81, "endLine": 85}
```

## Excerpt

<span id="rgbdns-frag-ea6bdb9d177f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ea6bdb9d177f: fn wildcard

```rust
    pub fn wildcard(&self) -> Self {
        let mut labels = self.0.clone();
        labels.insert(0, b"*".to_vec());
        Self(labels)
    }
```
