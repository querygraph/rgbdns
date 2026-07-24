---
type: "code-fragment"
fragment_id: "rgbdns-frag-db46c882156d"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "field"
kind: "fn"
start_line: 643
end_line: 647
---

# field

- Fragment ID: `rgbdns-frag-db46c882156d`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 643-647
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-db46c882156d", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-db46c882156d: fn field", "sourcePath": "src/zone.rs", "startLine": 643, "endLine": 647}
```

## Excerpt

<span id="rgbdns-frag-db46c882156d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-db46c882156d: fn field

```rust
fn field(f: &[String], i: usize) -> Result<&str> {
    f.get(i)
        .map(String::as_str)
        .ok_or_else(|| Error::InvalidRecord(format!("missing field {}", i + 1)))
}
```
