---
type: "code-fragment"
fragment_id: "rgbdns-frag-d43022c23aa8"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "is_subdomain_of"
kind: "fn"
start_line: 78
end_line: 92
---

# is_subdomain_of

- Fragment ID: `rgbdns-frag-d43022c23aa8`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 78-92
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d43022c23aa8", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "sourcePath": "src/name.rs", "startLine": 78, "endLine": 92}
```

## Excerpt

<span id="rgbdns-frag-d43022c23aa8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d43022c23aa8: fn is_subdomain_of

```rust
    pub fn is_subdomain_of(&self, other: &Self) -> bool {
        if self.0.len() < other.0.len() {
            return false;
        }
        self.0[self.0.len() - other.0.len()..]
            .iter()
            .zip(&other.0)
            .all(|(left, right)| {
                left.len() == right.len()
                    && left
                        .iter()
                        .zip(right)
                        .all(|(a, b)| a.eq_ignore_ascii_case(b))
            })
    }
```
