---
type: "code-fragment"
fragment_id: "rgbdns-frag-376a41ca433a"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "is_subdomain_of"
kind: "fn"
start_line: 78
end_line: 80
---

# is_subdomain_of

- Fragment ID: `rgbdns-frag-376a41ca433a`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 78-80
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-376a41ca433a", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-376a41ca433a: fn is_subdomain_of", "sourcePath": "src/name.rs", "startLine": 78, "endLine": 80}
```

## Excerpt

<span id="rgbdns-frag-376a41ca433a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-376a41ca433a: fn is_subdomain_of

```rust
    pub fn is_subdomain_of(&self, other: &Self) -> bool {
        self.0.len() >= other.0.len() && self.0[self.0.len() - other.0.len()..] == other.0
    }
```
