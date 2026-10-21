---
type: "code-fragment"
fragment_id: "rgbdns-frag-d5c190f439d7"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "serve_sockets"
kind: "fn"
start_line: 718
end_line: 752
---

# serve_sockets

- Fragment ID: `rgbdns-frag-d5c190f439d7`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 718-752
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d5c190f439d7", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 718, "endLine": 752}
```

## Excerpt

<span id="rgbdns-frag-d5c190f439d7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d5c190f439d7: fn serve_sockets

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
    let query_logger = QueryLogger { enabled: false };
    let stream_handler = allowed.map(|allowed| {
        let live = crate::acme::LiveZone::new((*zone).clone());
        authoritative_stream_handler(live, Some(Arc::new(allowed)), None, query_logger)
    });
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
                Some(client.ip()),
            )
        }),
        stream_handler,
    )
}

#[cfg(test)]
```
