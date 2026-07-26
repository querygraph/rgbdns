---
type: "code-fragment"
fragment_id: "rgbdns-frag-cf6623db375a"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "Error"
kind: "enum"
start_line: 28
end_line: 34
---

# Error

- Fragment ID: `rgbdns-frag-cf6623db375a`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 28-34
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-cf6623db375a", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-cf6623db375a: enum Error", "sourcePath": "src/lib.rs", "startLine": 28, "endLine": 34}
```

## Excerpt

<span id="rgbdns-frag-cf6623db375a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-cf6623db375a: enum Error

```rust
pub enum Error {
    Io(std::io::Error),
    Format(&'static str),
    InvalidName(String),
    InvalidRecord(String),
}

```
