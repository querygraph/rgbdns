---
type: "code-fragment"
fragment_id: "rgbdns-frag-1e7091c46f9a"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "serve"
kind: "fn"
start_line: 346
end_line: 356
---

# serve

- Fragment ID: `rgbdns-frag-1e7091c46f9a`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 346-356
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1e7091c46f9a", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-1e7091c46f9a: fn serve", "sourcePath": "src/server.rs", "startLine": 346, "endLine": 356}
```

## Excerpt

<span id="rgbdns-frag-1e7091c46f9a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1e7091c46f9a: fn serve

```rust
pub fn serve(zone: Zone, addr: &str) -> Result<()> {
    let zone = Arc::new(zone);
    crate::transport::serve(
        addr,
        Arc::new(move |wire, limit, client| {
            respond_over_transport(&zone, wire, limit, limit <= 4096, Some(client))
        }),
    )
}

#[cfg(test)]
```
