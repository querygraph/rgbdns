---
type: "code-fragment"
fragment_id: "rgbdns-frag-a0394db83dec"
source_path: "src/transport.rs"
code_note: "DNS from First Principles/Code/src/transport.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "serve_udp"
kind: "fn"
start_line: 66
end_line: 76
---

# serve_udp

- Fragment ID: `rgbdns-frag-a0394db83dec`
- Source file: [[DNS from First Principles/Code/src/transport.rs.source|src/transport.rs]]
- Lines: 66-76
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a0394db83dec", "codeNote": "DNS from First Principles/Code/src/transport.rs.source", "heading": "rgbdns-frag-a0394db83dec: fn serve_udp", "sourcePath": "src/transport.rs", "startLine": 66, "endLine": 76}
```

## Excerpt

<span id="rgbdns-frag-a0394db83dec" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a0394db83dec: fn serve_udp

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
