---
type: "code-fragment"
fragment_id: "rgbdns-frag-4e2018a99a96"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "wildcard"
kind: "fn"
start_line: 93
end_line: 97
---

# wildcard

- Fragment ID: `rgbdns-frag-4e2018a99a96`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 93-97
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4e2018a99a96", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "sourcePath": "src/name.rs", "startLine": 93, "endLine": 97}
```

## Excerpt

<span id="rgbdns-frag-4e2018a99a96" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4e2018a99a96: fn wildcard

```rust
    pub fn wildcard(&self) -> Self {
        let mut labels = self.0.clone();
        labels.insert(0, b"*".to_vec());
        Self(labels)
    }
```
