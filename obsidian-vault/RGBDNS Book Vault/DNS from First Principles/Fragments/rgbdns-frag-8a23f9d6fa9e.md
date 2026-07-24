---
type: "code-fragment"
fragment_id: "rgbdns-frag-8a23f9d6fa9e"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "serve_sockets"
kind: "fn"
start_line: 357
end_line: 368
---

# serve_sockets

- Fragment ID: `rgbdns-frag-8a23f9d6fa9e`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 357-368
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-8a23f9d6fa9e", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-8a23f9d6fa9e: fn serve_sockets", "sourcePath": "src/server.rs", "startLine": 357, "endLine": 368}
```

## Excerpt

<span id="rgbdns-frag-8a23f9d6fa9e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8a23f9d6fa9e: fn serve_sockets

```rust
fn serve_sockets(zone: Zone, udp: std::net::UdpSocket, tcp: std::net::TcpListener) -> Result<()> {
    let zone = Arc::new(zone);
    crate::transport::serve_sockets(
        udp,
        tcp,
        Arc::new(move |wire, limit, client| {
            respond_over_transport(&zone, wire, limit, limit <= 4096, Some(client))
        }),
    )
}

#[cfg(test)]
```
