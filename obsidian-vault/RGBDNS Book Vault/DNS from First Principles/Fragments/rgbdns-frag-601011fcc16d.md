---
type: "code-fragment"
fragment_id: "rgbdns-frag-601011fcc16d"
source_path: "src/transport.rs"
code_note: "DNS from First Principles/Code/src/transport.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "serve_udp"
kind: "fn"
start_line: 54
end_line: 64
---

# serve_udp

- Fragment ID: `rgbdns-frag-601011fcc16d`
- Source file: [[DNS from First Principles/Code/src/transport.rs.source|src/transport.rs]]
- Lines: 54-64
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-601011fcc16d", "codeNote": "DNS from First Principles/Code/src/transport.rs.source", "heading": "rgbdns-frag-601011fcc16d: fn serve_udp", "sourcePath": "src/transport.rs", "startLine": 54, "endLine": 64}
```

## Excerpt

<span id="rgbdns-frag-601011fcc16d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-601011fcc16d: fn serve_udp

```rust
fn serve_udp(socket: UdpSocket, handler: &Arc<Handler>) {
    let mut packet = [0; u16::MAX as usize];
    loop {
        if let Ok((length, peer)) = socket.recv_from(&mut packet)
            && let Ok(response) = handler(&packet[..length], 4096, peer.ip())
        {
            let _ = socket.send_to(&response, peer);
        }
    }
}

```
