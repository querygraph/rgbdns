---
type: "code-fragment"
fragment_id: "rgbdns-frag-57473282a4ff"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "CacheEntry"
kind: "struct"
start_line: 22
end_line: 26
---

# CacheEntry

- Fragment ID: `rgbdns-frag-57473282a4ff`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 22-26
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-57473282a4ff", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "heading": "rgbdns-frag-57473282a4ff: struct CacheEntry", "sourcePath": "src/aname.rs", "startLine": 22, "endLine": 26}
```

## Excerpt

<span id="rgbdns-frag-57473282a4ff" class="rgbdns-fragment-target"></span>
### rgbdns-frag-57473282a4ff: struct CacheEntry

```rust
struct CacheEntry {
    data: Vec<RData>,
    expires: Instant,
}

```
