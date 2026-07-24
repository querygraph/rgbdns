---
type: "code-fragment"
fragment_id: "rgbdns-frag-a504095bfade"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "Error"
kind: "enum"
start_line: 27
end_line: 33
---

# Error

- Fragment ID: `rgbdns-frag-a504095bfade`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 27-33
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a504095bfade", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-a504095bfade: enum Error", "sourcePath": "src/lib.rs", "startLine": 27, "endLine": 33}
```

## Excerpt

<span id="rgbdns-frag-a504095bfade" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a504095bfade: enum Error

```rust
pub enum Error {
    Io(std::io::Error),
    Format(&'static str),
    InvalidName(String),
    InvalidRecord(String),
}

```
