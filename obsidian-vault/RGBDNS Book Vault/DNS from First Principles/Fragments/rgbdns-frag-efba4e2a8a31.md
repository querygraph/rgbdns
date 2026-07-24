---
type: "code-fragment"
fragment_id: "rgbdns-frag-efba4e2a8a31"
source_path: "src/tinydns_edit.rs"
code_note: "DNS from First Principles/Code/src/tinydns_edit.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "Mode"
kind: "enum"
start_line: 12
end_line: 21
---

# Mode

- Fragment ID: `rgbdns-frag-efba4e2a8a31`
- Source file: [[DNS from First Principles/Code/src/tinydns_edit.rs.source|src/tinydns_edit.rs]]
- Lines: 12-21
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-efba4e2a8a31", "codeNote": "DNS from First Principles/Code/src/tinydns_edit.rs.source", "heading": "rgbdns-frag-efba4e2a8a31: enum Mode", "sourcePath": "src/tinydns_edit.rs", "startLine": 12, "endLine": 21}
```

## Excerpt

<span id="rgbdns-frag-efba4e2a8a31" class="rgbdns-fragment-target"></span>
### rgbdns-frag-efba4e2a8a31: enum Mode

```rust
pub enum Mode {
    Ns,
    ChildNs,
    Host,
    Alias,
    Mx,
    Host6,
    Alias6,
}

```
