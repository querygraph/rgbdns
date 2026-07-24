---
type: "code-fragment"
fragment_id: "rgbdns-frag-961bde153386"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "Result"
kind: "type"
start_line: 50
end_line: 52
---

# Result

- Fragment ID: `rgbdns-frag-961bde153386`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 50-52
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-961bde153386", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-961bde153386: type Result", "sourcePath": "src/lib.rs", "startLine": 50, "endLine": 52}
```

## Excerpt

<span id="rgbdns-frag-961bde153386" class="rgbdns-fragment-target"></span>
### rgbdns-frag-961bde153386: type Result

```rust
pub type Result<T> = std::result::Result<T, Error>;

/// Constructs a listen address without ambiguous IPv6 string concatenation.
```
