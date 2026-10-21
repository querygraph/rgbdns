---
type: "code-fragment"
fragment_id: "rgbdns-frag-7d5ad2f48fd1"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "respond"
kind: "fn"
start_line: 136
end_line: 143
---

# respond

- Fragment ID: `rgbdns-frag-7d5ad2f48fd1`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 136-143
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7d5ad2f48fd1", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 136, "endLine": 143}
```

## Excerpt

<span id="rgbdns-frag-7d5ad2f48fd1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7d5ad2f48fd1: fn respond

```rust
pub fn respond(zone: &Zone, wire: &[u8], transport_limit: usize) -> Result<Vec<u8>> {
    let resolver = zone
        .has_anames()
        .then(crate::aname::Resolver::from_system)
        .transpose()?;
    respond_over_transport(zone, resolver.as_ref(), wire, transport_limit, true, None)
}

```
