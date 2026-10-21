---
type: "code-fragment"
fragment_id: "rgbdns-frag-2203e709c4c4"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "CacheEntry"
kind: "enum"
start_line: 24
end_line: 29
---

# CacheEntry

- Fragment ID: `rgbdns-frag-2203e709c4c4`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 24-29
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-2203e709c4c4", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "sourcePath": "src/aname.rs", "startLine": 24, "endLine": 29}
```

## Excerpt

<span id="rgbdns-frag-2203e709c4c4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2203e709c4c4: enum CacheEntry

```rust
enum CacheEntry {
    Ready { data: Vec<RData>, expires: Instant },
    Loading,
    Failed { expires: Instant },
}

```
