---
type: "code-fragment"
fragment_id: "rgbdns-frag-4e4194bd150f"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "without_wildcard"
kind: "fn"
start_line: 105
end_line: 110
---

# without_wildcard

- Fragment ID: `rgbdns-frag-4e4194bd150f`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 105-110
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4e4194bd150f", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-4e4194bd150f: fn without_wildcard", "sourcePath": "src/name.rs", "startLine": 105, "endLine": 110}
```

## Excerpt

<span id="rgbdns-frag-4e4194bd150f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4e4194bd150f: fn without_wildcard

```rust
    pub(crate) fn without_wildcard(&self) -> Option<Self> {
        self.0
            .first()
            .filter(|label| label.as_slice() == b"*")
            .map(|_| Self(self.0[1..].to_vec()))
    }
```
