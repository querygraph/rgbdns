---
type: "code-fragment"
fragment_id: "rgbdns-frag-a20c06228073"
source_path: "src/transport.rs"
code_note: "DNS from First Principles/Code/src/transport.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "serve_udp"
kind: "fn"
start_line: 67
end_line: 77
---

# serve_udp

- Fragment ID: `rgbdns-frag-a20c06228073`
- Source file: [[DNS from First Principles/Code/src/transport.rs.source|src/transport.rs]]
- Lines: 67-77
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a20c06228073", "codeNote": "DNS from First Principles/Code/src/transport.rs.source", "sourcePath": "src/transport.rs", "startLine": 67, "endLine": 77}
```

## Excerpt

<span id="rgbdns-frag-a20c06228073" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a20c06228073: fn serve_udp

```rust
fn serve_udp(socket: UdpSocket, handler: &Arc<Handler>) {
    let mut packet = [0; u16::MAX as usize];
    loop {
        if let Ok((length, peer)) = socket.recv_from(&mut packet)
            && let Ok(response) = handler(&packet[..length], 4096, peer)
        {
            let _ = socket.send_to(&response, peer);
        }
    }
}

```
