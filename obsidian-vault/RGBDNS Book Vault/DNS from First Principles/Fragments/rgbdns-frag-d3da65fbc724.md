---
type: "code-fragment"
fragment_id: "rgbdns-frag-d3da65fbc724"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "respond"
kind: "fn"
start_line: 12
end_line: 19
---

# respond

- Fragment ID: `rgbdns-frag-d3da65fbc724`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 12-19
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d3da65fbc724", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-d3da65fbc724: fn respond", "sourcePath": "src/server.rs", "startLine": 12, "endLine": 19}
```

## Excerpt

<span id="rgbdns-frag-d3da65fbc724" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d3da65fbc724: fn respond

```rust
pub fn respond(zone: &Zone, wire: &[u8], transport_limit: usize) -> Result<Vec<u8>> {
    let resolver = zone
        .has_anames()
        .then(crate::aname::Resolver::from_system)
        .transpose()?;
    respond_over_transport(zone, resolver.as_ref(), wire, transport_limit, true, None)
}

```
