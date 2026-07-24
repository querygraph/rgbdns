---
type: "code-fragment"
fragment_id: "rgbdns-frag-a0dfeb34bf30"
source_path: "src/transport.rs"
code_note: "DNS from First Principles/Code/src/transport.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "serve_tcp_connection"
kind: "fn"
start_line: 65
end_line: 88
---

# serve_tcp_connection

- Fragment ID: `rgbdns-frag-a0dfeb34bf30`
- Source file: [[DNS from First Principles/Code/src/transport.rs.source|src/transport.rs]]
- Lines: 65-88
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a0dfeb34bf30", "codeNote": "DNS from First Principles/Code/src/transport.rs.source", "heading": "rgbdns-frag-a0dfeb34bf30: fn serve_tcp_connection", "sourcePath": "src/transport.rs", "startLine": 65, "endLine": 88}
```

## Excerpt

<span id="rgbdns-frag-a0dfeb34bf30" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a0dfeb34bf30: fn serve_tcp_connection

```rust
fn serve_tcp_connection(stream: &mut TcpStream, client: IpAddr, handler: &Arc<Handler>) {
    loop {
        let mut length = [0; 2];
        if stream.read_exact(&mut length).is_err() {
            return;
        }
        let mut packet = vec![0; u16::from_be_bytes(length) as usize];
        if stream.read_exact(&mut packet).is_err() {
            return;
        }
        let Ok(response) = handler(&packet, u16::MAX as usize, client) else {
            continue;
        };
        let Ok(response_length) = u16::try_from(response.len()) else {
            return;
        };
        let mut framed = Vec::with_capacity(response.len() + 2);
        framed.extend(response_length.to_be_bytes());
        framed.extend(response);
        if stream.write_all(&framed).is_err() {
            return;
        }
    }
}
```
