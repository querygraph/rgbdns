---
type: "code-fragment"
fragment_id: "rgbdns-frag-31d10be4bc98"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "serve"
kind: "fn"
start_line: 387
end_line: 423
---

# serve

- Fragment ID: `rgbdns-frag-31d10be4bc98`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 387-423
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-31d10be4bc98", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-31d10be4bc98: fn serve", "sourcePath": "src/server.rs", "startLine": 387, "endLine": 423}
```

## Excerpt

<span id="rgbdns-frag-31d10be4bc98" class="rgbdns-fragment-target"></span>
### rgbdns-frag-31d10be4bc98: fn serve

```rust
pub fn serve(zone: Zone, addr: &str) -> Result<()> {
    let resolver = zone
        .has_anames()
        .then(crate::aname::Resolver::from_system)
        .transpose()?
        .map(Arc::new);
    let zone = Arc::new(zone);
    let allowed = std::env::var("ALLOW_NETS")
        .ok()
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::parse::<ipnet::IpNet>)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|_| Error::Format("invalid ALLOW_NETS"))
        })
        .transpose()?
        .map(Arc::new);
    let stream_handler = allowed.map(|allowed| axfr_stream_handler(zone.clone(), allowed));
    crate::transport::serve(
        addr,
        Arc::new(move |wire, limit, client| {
            respond_over_transport(
                &zone,
                resolver.as_deref(),
                wire,
                limit,
                limit <= 4096,
                Some(client),
            )
        }),
        stream_handler,
    )
}

```
