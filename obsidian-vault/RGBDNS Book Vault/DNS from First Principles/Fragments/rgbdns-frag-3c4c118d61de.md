---
type: "code-fragment"
fragment_id: "rgbdns-frag-3c4c118d61de"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "number_or"
kind: "fn"
start_line: 989
end_line: 994
---

# number_or

- Fragment ID: `rgbdns-frag-3c4c118d61de`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 989-994
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3c4c118d61de", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 989, "endLine": 994}
```

## Excerpt

<span id="rgbdns-frag-3c4c118d61de" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3c4c118d61de: fn number_or

```rust
fn number_or<T: FromStr + Copy>(fields: &[String], index: usize, default: T) -> T {
    field_opt(fields, index)
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}
```
