---
type: "code-fragment"
fragment_id: "rgbdns-frag-52c67c7541a7"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "canonical_name_cmp"
kind: "fn"
start_line: 596
end_line: 612
---

# canonical_name_cmp

- Fragment ID: `rgbdns-frag-52c67c7541a7`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 596-612
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-52c67c7541a7", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 596, "endLine": 612}
```

## Excerpt

<span id="rgbdns-frag-52c67c7541a7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-52c67c7541a7: fn canonical_name_cmp

```rust
fn canonical_name_cmp(left: &Name, right: &Name) -> Ordering {
    let mut left = left.labels().collect::<Vec<_>>();
    let mut right = right.labels().collect::<Vec<_>>();
    left.reverse();
    right.reverse();
    for (left, right) in left.iter().zip(&right) {
        let ordering = left
            .iter()
            .map(u8::to_ascii_lowercase)
            .cmp(right.iter().map(u8::to_ascii_lowercase));
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    left.len().cmp(&right.len())
}

```
