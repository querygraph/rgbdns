---
type: "code-fragment"
fragment_id: "rgbdns-frag-4373cfd6a122"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "Result"
kind: "type"
start_line: 51
end_line: 53
---

# Result

- Fragment ID: `rgbdns-frag-4373cfd6a122`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 51-53
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4373cfd6a122", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-4373cfd6a122: type Result", "sourcePath": "src/lib.rs", "startLine": 51, "endLine": 53}
```

## Excerpt

<span id="rgbdns-frag-4373cfd6a122" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4373cfd6a122: type Result

```rust
pub type Result<T> = std::result::Result<T, Error>;

/// Constructs a listen address without ambiguous IPv6 string concatenation.
```
