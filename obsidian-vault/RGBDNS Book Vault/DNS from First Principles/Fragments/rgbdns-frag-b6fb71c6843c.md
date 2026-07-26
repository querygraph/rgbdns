---
type: "code-fragment"
fragment_id: "rgbdns-frag-b6fb71c6843c"
source_path: "src/transport.rs"
code_note: "DNS from First Principles/Code/src/transport.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "serve"
kind: "fn"
start_line: 18
end_line: 30
---

# serve

- Fragment ID: `rgbdns-frag-b6fb71c6843c`
- Source file: [[DNS from First Principles/Code/src/transport.rs.source|src/transport.rs]]
- Lines: 18-30
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b6fb71c6843c", "codeNote": "DNS from First Principles/Code/src/transport.rs.source", "heading": "rgbdns-frag-b6fb71c6843c: fn serve", "sourcePath": "src/transport.rs", "startLine": 18, "endLine": 30}
```

## Excerpt

<span id="rgbdns-frag-b6fb71c6843c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b6fb71c6843c: fn serve

```rust
pub(crate) fn serve(
    address: &str,
    handler: Arc<Handler>,
    stream_handler: Option<Arc<StreamHandler>>,
) -> Result<()> {
    serve_sockets(
        UdpSocket::bind(address)?,
        TcpListener::bind(address)?,
        handler,
        stream_handler,
    )
}

```
