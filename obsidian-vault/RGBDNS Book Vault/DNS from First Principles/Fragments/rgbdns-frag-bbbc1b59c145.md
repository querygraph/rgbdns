---
type: "code-fragment"
fragment_id: "rgbdns-frag-bbbc1b59c145"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "field_opt"
kind: "fn"
start_line: 718
end_line: 720
---

# field_opt

- Fragment ID: `rgbdns-frag-bbbc1b59c145`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 718-720
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-bbbc1b59c145", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-bbbc1b59c145: fn field_opt", "sourcePath": "src/zone.rs", "startLine": 718, "endLine": 720}
```

## Excerpt

<span id="rgbdns-frag-bbbc1b59c145" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bbbc1b59c145: fn field_opt

```rust
fn field_opt(f: &[String], i: usize) -> Option<&str> {
    f.get(i).map(String::as_str)
}
```
