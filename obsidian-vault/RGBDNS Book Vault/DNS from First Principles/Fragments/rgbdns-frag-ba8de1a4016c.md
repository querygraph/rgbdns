---
type: "code-fragment"
fragment_id: "rgbdns-frag-ba8de1a4016c"
source_path: "src/tinydns_edit.rs"
code_note: "DNS from First Principles/Code/src/tinydns_edit.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "name_field"
kind: "fn"
start_line: 198
end_line: 201
---

# name_field

- Fragment ID: `rgbdns-frag-ba8de1a4016c`
- Source file: [[DNS from First Principles/Code/src/tinydns_edit.rs.source|src/tinydns_edit.rs]]
- Lines: 198-201
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ba8de1a4016c", "codeNote": "DNS from First Principles/Code/src/tinydns_edit.rs.source", "heading": "rgbdns-frag-ba8de1a4016c: fn name_field", "sourcePath": "src/tinydns_edit.rs", "startLine": 198, "endLine": 201}
```

## Excerpt

<span id="rgbdns-frag-ba8de1a4016c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ba8de1a4016c: fn name_field

```rust
fn name_field(fields: &[String], index: usize) -> Option<Name> {
    fields.get(index)?.parse().ok()
}

```
