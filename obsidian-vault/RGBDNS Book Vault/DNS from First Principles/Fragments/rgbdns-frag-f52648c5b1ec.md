---
type: "code-fragment"
fragment_id: "rgbdns-frag-f52648c5b1ec"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "number_or"
kind: "fn"
start_line: 651
end_line: 656
---

# number_or

- Fragment ID: `rgbdns-frag-f52648c5b1ec`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 651-656
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f52648c5b1ec", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-f52648c5b1ec: fn number_or", "sourcePath": "src/zone.rs", "startLine": 651, "endLine": 656}
```

## Excerpt

<span id="rgbdns-frag-f52648c5b1ec" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f52648c5b1ec: fn number_or

```rust
fn number_or<T: FromStr + Copy>(fields: &[String], index: usize, default: T) -> T {
    field_opt(fields, index)
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}
```
