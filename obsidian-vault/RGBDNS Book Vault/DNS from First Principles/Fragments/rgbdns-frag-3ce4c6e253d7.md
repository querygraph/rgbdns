---
type: "code-fragment"
fragment_id: "rgbdns-frag-3ce4c6e253d7"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "without_wildcard"
kind: "fn"
start_line: 117
end_line: 122
---

# without_wildcard

- Fragment ID: `rgbdns-frag-3ce4c6e253d7`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 117-122
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3ce4c6e253d7", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "sourcePath": "src/name.rs", "startLine": 117, "endLine": 122}
```

## Excerpt

<span id="rgbdns-frag-3ce4c6e253d7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3ce4c6e253d7: fn without_wildcard

```rust
    pub(crate) fn without_wildcard(&self) -> Option<Self> {
        self.0
            .first()
            .filter(|label| label.as_slice() == b"*")
            .map(|_| Self(self.0[1..].to_vec()))
    }
```
