---
type: "code-fragment"
fragment_id: "rgbdns-frag-e7b7438516db"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "Result"
kind: "type"
start_line: 53
end_line: 55
---

# Result

- Fragment ID: `rgbdns-frag-e7b7438516db`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 53-55
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e7b7438516db", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "sourcePath": "src/lib.rs", "startLine": 53, "endLine": 55}
```

## Excerpt

<span id="rgbdns-frag-e7b7438516db" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e7b7438516db: type Result

```rust
pub type Result<T> = std::result::Result<T, Error>;

/// Constructs a listen address without ambiguous IPv6 string concatenation.
```
