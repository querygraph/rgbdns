---
type: "code-fragment"
fragment_id: "rgbdns-frag-7aee695735c0"
source_path: "src/transport.rs"
code_note: "DNS from First Principles/Code/src/transport.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "serve"
kind: "fn"
start_line: 19
end_line: 31
---

# serve

- Fragment ID: `rgbdns-frag-7aee695735c0`
- Source file: [[DNS from First Principles/Code/src/transport.rs.source|src/transport.rs]]
- Lines: 19-31
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7aee695735c0", "codeNote": "DNS from First Principles/Code/src/transport.rs.source", "sourcePath": "src/transport.rs", "startLine": 19, "endLine": 31}
```

## Excerpt

<span id="rgbdns-frag-7aee695735c0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7aee695735c0: fn serve

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
