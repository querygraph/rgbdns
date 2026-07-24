---
type: "code-fragment"
fragment_id: "rgbdns-frag-02bc3279cfc3"
source_path: "src/transport.rs"
code_note: "DNS from First Principles/Code/src/transport.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "serve"
kind: "fn"
start_line: 17
end_line: 24
---

# serve

- Fragment ID: `rgbdns-frag-02bc3279cfc3`
- Source file: [[DNS from First Principles/Code/src/transport.rs.source|src/transport.rs]]
- Lines: 17-24
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-02bc3279cfc3", "codeNote": "DNS from First Principles/Code/src/transport.rs.source", "heading": "rgbdns-frag-02bc3279cfc3: fn serve", "sourcePath": "src/transport.rs", "startLine": 17, "endLine": 24}
```

## Excerpt

<span id="rgbdns-frag-02bc3279cfc3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-02bc3279cfc3: fn serve

```rust
pub(crate) fn serve(address: &str, handler: Arc<Handler>) -> Result<()> {
    serve_sockets(
        UdpSocket::bind(address)?,
        TcpListener::bind(address)?,
        handler,
    )
}

```
