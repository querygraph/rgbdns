---
type: "code-fragment"
fragment_id: "rgbdns-frag-3570b5155e9d"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "Resolver"
kind: "struct"
start_line: 27
end_line: 31
---

# Resolver

- Fragment ID: `rgbdns-frag-3570b5155e9d`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 27-31
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3570b5155e9d", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "heading": "rgbdns-frag-3570b5155e9d: struct Resolver", "sourcePath": "src/aname.rs", "startLine": 27, "endLine": 31}
```

## Excerpt

<span id="rgbdns-frag-3570b5155e9d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3570b5155e9d: struct Resolver

```rust
pub(crate) struct Resolver {
    servers: Vec<SocketAddr>,
    cache: Mutex<HashMap<CacheKey, CacheEntry>>,
}

```
