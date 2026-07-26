---
type: "code-fragment"
fragment_id: "rgbdns-frag-053b53ae57e0"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "serve_sockets"
kind: "fn"
start_line: 445
end_line: 476
---

# serve_sockets

- Fragment ID: `rgbdns-frag-053b53ae57e0`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 445-476
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-053b53ae57e0", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-053b53ae57e0: fn serve_sockets", "sourcePath": "src/server.rs", "startLine": 445, "endLine": 476}
```

## Excerpt

<span id="rgbdns-frag-053b53ae57e0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-053b53ae57e0: fn serve_sockets

```rust
fn serve_sockets(
    zone: Zone,
    udp: std::net::UdpSocket,
    tcp: std::net::TcpListener,
    allowed: Option<Vec<ipnet::IpNet>>,
) -> Result<()> {
    let resolver = zone
        .has_anames()
        .then(crate::aname::Resolver::from_system)
        .transpose()?
        .map(Arc::new);
    let zone = Arc::new(zone);
    let stream_handler =
        allowed.map(|allowed| axfr_stream_handler(zone.clone(), Arc::new(allowed)));
    crate::transport::serve_sockets(
        udp,
        tcp,
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

#[cfg(test)]
```
