---
type: "code-fragment"
fragment_id: "rgbdns-frag-8a1e435a8dd9"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "Reader"
kind: "struct"
start_line: 897
end_line: 901
---

# Reader

- Fragment ID: `rgbdns-frag-8a1e435a8dd9`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 897-901
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-8a1e435a8dd9", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 897, "endLine": 901}
```

## Excerpt

<span id="rgbdns-frag-8a1e435a8dd9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8a1e435a8dd9: struct Reader

```rust
struct Reader<'a> {
    wire: &'a [u8],
    position: usize,
}

```
