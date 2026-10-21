---
type: "code-fragment"
fragment_id: "rgbdns-frag-108ded6ad017"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "unix_now"
kind: "fn"
start_line: 1155
end_line: 1161
---

# unix_now

- Fragment ID: `rgbdns-frag-108ded6ad017`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 1155-1161
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-108ded6ad017", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 1155, "endLine": 1161}
```

## Excerpt

<span id="rgbdns-frag-108ded6ad017" class="rgbdns-fragment-target"></span>
### rgbdns-frag-108ded6ad017: fn unix_now

```rust
fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

```
