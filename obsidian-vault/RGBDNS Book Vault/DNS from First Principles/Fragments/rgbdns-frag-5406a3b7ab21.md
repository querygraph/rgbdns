---
type: "code-fragment"
fragment_id: "rgbdns-frag-5406a3b7ab21"
source_path: "src/client.rs"
code_note: "DNS from First Principles/Code/src/client.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "udp_query"
kind: "fn"
start_line: 94
end_line: 109
---

# udp_query

- Fragment ID: `rgbdns-frag-5406a3b7ab21`
- Source file: [[DNS from First Principles/Code/src/client.rs.source|src/client.rs]]
- Lines: 94-109
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5406a3b7ab21", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "heading": "rgbdns-frag-5406a3b7ab21: fn udp_query", "sourcePath": "src/client.rs", "startLine": 94, "endLine": 109}
```

## Excerpt

<span id="rgbdns-frag-5406a3b7ab21" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5406a3b7ab21: fn udp_query

```rust
fn udp_query(server: SocketAddr, wire: &[u8], id: u16, question: &Question) -> Result<Message> {
    let bind = if server.is_ipv4() {
        "0.0.0.0:0"
    } else {
        "[::]:0"
    };
    let socket = UdpSocket::bind(bind)?;
    socket.set_read_timeout(Some(TIMEOUT))?;
    socket.set_write_timeout(Some(TIMEOUT))?;
    socket.connect(server)?;
    socket.send(wire)?;
    let mut response = [0; 65535];
    let length = socket.recv(&mut response)?;
    validate(Message::decode(&response[..length])?, id, question, false)
}

```
