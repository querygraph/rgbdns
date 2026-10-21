---
type: "code-fragment"
fragment_id: "rgbdns-frag-27b46e14fabd"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "CacheKey"
kind: "struct"
start_line: 18
end_line: 23
---

# CacheKey

- Fragment ID: `rgbdns-frag-27b46e14fabd`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 18-23
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-27b46e14fabd", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "sourcePath": "src/aname.rs", "startLine": 18, "endLine": 23}
```

## Excerpt

<span id="rgbdns-frag-27b46e14fabd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-27b46e14fabd: struct CacheKey

```rust
struct CacheKey {
    target: Name,
    record_type: RecordType,
}

#[derive(Clone, Debug)]
```
