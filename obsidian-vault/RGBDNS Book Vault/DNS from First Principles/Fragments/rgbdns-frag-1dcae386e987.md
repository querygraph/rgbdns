---
type: "code-fragment"
fragment_id: "rgbdns-frag-1dcae386e987"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "field"
kind: "fn"
start_line: 713
end_line: 717
---

# field

- Fragment ID: `rgbdns-frag-1dcae386e987`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 713-717
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1dcae386e987", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-1dcae386e987: fn field", "sourcePath": "src/zone.rs", "startLine": 713, "endLine": 717}
```

## Excerpt

<span id="rgbdns-frag-1dcae386e987" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1dcae386e987: fn field

```rust
fn field(f: &[String], i: usize) -> Result<&str> {
    f.get(i)
        .map(String::as_str)
        .ok_or_else(|| Error::InvalidRecord(format!("missing field {}", i + 1)))
}
```
