---
type: "code-fragment"
fragment_id: "rgbdns-frag-ad7b342aa5e0"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "eq"
kind: "fn"
start_line: 18
end_line: 29
---

# eq

- Fragment ID: `rgbdns-frag-ad7b342aa5e0`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 18-29
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ad7b342aa5e0", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-ad7b342aa5e0: fn eq", "sourcePath": "src/name.rs", "startLine": 18, "endLine": 29}
```

## Excerpt

<span id="rgbdns-frag-ad7b342aa5e0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ad7b342aa5e0: fn eq

```rust
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
            && self.0.iter().zip(&other.0).all(|(left, right)| {
                left.len() == right.len()
                    && left
                        .iter()
                        .zip(right)
                        .all(|(a, b)| a.eq_ignore_ascii_case(b))
            })
    }
}

```
