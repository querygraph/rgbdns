---
type: "code-fragment"
fragment_id: "rgbdns-frag-d6016dd9041c"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "field_opt"
kind: "fn"
start_line: 648
end_line: 650
---

# field_opt

- Fragment ID: `rgbdns-frag-d6016dd9041c`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 648-650
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d6016dd9041c", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-d6016dd9041c: fn field_opt", "sourcePath": "src/zone.rs", "startLine": 648, "endLine": 650}
```

## Excerpt

<span id="rgbdns-frag-d6016dd9041c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d6016dd9041c: fn field_opt

```rust
fn field_opt(f: &[String], i: usize) -> Option<&str> {
    f.get(i).map(String::as_str)
}
```
