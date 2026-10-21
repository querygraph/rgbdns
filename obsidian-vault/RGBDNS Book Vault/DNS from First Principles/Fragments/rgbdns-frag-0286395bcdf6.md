---
type: "code-fragment"
fragment_id: "rgbdns-frag-0286395bcdf6"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "Resolver"
kind: "struct"
start_line: 30
end_line: 35
---

# Resolver

- Fragment ID: `rgbdns-frag-0286395bcdf6`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 30-35
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0286395bcdf6", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "sourcePath": "src/aname.rs", "startLine": 30, "endLine": 35}
```

## Excerpt

<span id="rgbdns-frag-0286395bcdf6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0286395bcdf6: struct Resolver

```rust
pub(crate) struct Resolver {
    servers: Vec<SocketAddr>,
    cache: Mutex<HashMap<CacheKey, CacheEntry>>,
    cache_changed: Condvar,
}

```
