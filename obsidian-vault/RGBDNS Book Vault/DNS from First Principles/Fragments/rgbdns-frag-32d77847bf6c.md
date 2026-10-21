---
type: "code-fragment"
fragment_id: "rgbdns-frag-32d77847bf6c"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "Error"
kind: "enum"
start_line: 30
end_line: 36
---

# Error

- Fragment ID: `rgbdns-frag-32d77847bf6c`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 30-36
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-32d77847bf6c", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "sourcePath": "src/lib.rs", "startLine": 30, "endLine": 36}
```

## Excerpt

<span id="rgbdns-frag-32d77847bf6c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-32d77847bf6c: enum Error

```rust
pub enum Error {
    Io(std::io::Error),
    Format(&'static str),
    InvalidName(String),
    InvalidRecord(String),
}

```
