---
type: "code-fragment"
fragment_id: "rgbdns-frag-e0990e79e0f5"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "respond_from"
kind: "fn"
start_line: 20
end_line: 39
---

# respond_from

- Fragment ID: `rgbdns-frag-e0990e79e0f5`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 20-39
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e0990e79e0f5", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-e0990e79e0f5: fn respond_from", "sourcePath": "src/server.rs", "startLine": 20, "endLine": 39}
```

## Excerpt

<span id="rgbdns-frag-e0990e79e0f5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e0990e79e0f5: fn respond_from

```rust
pub fn respond_from(
    zone: &Zone,
    wire: &[u8],
    transport_limit: usize,
    client: IpAddr,
) -> Result<Vec<u8>> {
    let resolver = zone
        .has_anames()
        .then(crate::aname::Resolver::from_system)
        .transpose()?;
    respond_over_transport(
        zone,
        resolver.as_ref(),
        wire,
        transport_limit,
        true,
        Some(client),
    )
}

```
