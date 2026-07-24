---
type: "code-fragment"
fragment_id: "rgbdns-frag-0dc862d179e7"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "cmp"
kind: "fn"
start_line: 51
end_line: 64
---

# cmp

- Fragment ID: `rgbdns-frag-0dc862d179e7`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 51-64
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0dc862d179e7", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-0dc862d179e7: fn cmp", "sourcePath": "src/name.rs", "startLine": 51, "endLine": 64}
```

## Excerpt

<span id="rgbdns-frag-0dc862d179e7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0dc862d179e7: fn cmp

```rust
    fn cmp(&self, other: &Self) -> Ordering {
        for (left, right) in self.0.iter().zip(&other.0) {
            let ordering = left
                .iter()
                .map(u8::to_ascii_lowercase)
                .cmp(right.iter().map(u8::to_ascii_lowercase));
            if ordering != Ordering::Equal {
                return ordering;
            }
        }
        self.0.len().cmp(&other.0.len())
    }
}

```
