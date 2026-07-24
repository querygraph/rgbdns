---
type: "code-fragment"
fragment_id: "rgbdns-frag-9dd508018661"
source_path: "src/tinydns_edit.rs"
code_note: "DNS from First Principles/Code/src/tinydns_edit.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "number"
kind: "fn"
start_line: 202
end_line: 208
---

# number

- Fragment ID: `rgbdns-frag-9dd508018661`
- Source file: [[DNS from First Principles/Code/src/tinydns_edit.rs.source|src/tinydns_edit.rs]]
- Lines: 202-208
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9dd508018661", "codeNote": "DNS from First Principles/Code/src/tinydns_edit.rs.source", "heading": "rgbdns-frag-9dd508018661: fn number", "sourcePath": "src/tinydns_edit.rs", "startLine": 202, "endLine": 208}
```

## Excerpt

<span id="rgbdns-frag-9dd508018661" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9dd508018661: fn number

```rust
fn number(fields: &[String], index: usize, default: u32) -> u32 {
    fields
        .get(index)
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

```
