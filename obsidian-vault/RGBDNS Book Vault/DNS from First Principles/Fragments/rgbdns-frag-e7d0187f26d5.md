---
type: "code-fragment"
fragment_id: "rgbdns-frag-e7d0187f26d5"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "number_or"
kind: "fn"
start_line: 721
end_line: 726
---

# number_or

- Fragment ID: `rgbdns-frag-e7d0187f26d5`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 721-726
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e7d0187f26d5", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-e7d0187f26d5: fn number_or", "sourcePath": "src/zone.rs", "startLine": 721, "endLine": 726}
```

## Excerpt

<span id="rgbdns-frag-e7d0187f26d5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e7d0187f26d5: fn number_or

```rust
fn number_or<T: FromStr + Copy>(fields: &[String], index: usize, default: T) -> T {
    field_opt(fields, index)
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}
```
