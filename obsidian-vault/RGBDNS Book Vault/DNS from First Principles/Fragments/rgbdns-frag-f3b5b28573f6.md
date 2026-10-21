---
type: "code-fragment"
fragment_id: "rgbdns-frag-f3b5b28573f6"
source_path: "src/transport.rs"
code_note: "DNS from First Principles/Code/src/transport.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "serve_tcp_connection"
kind: "fn"
start_line: 78
end_line: 115
---

# serve_tcp_connection

- Fragment ID: `rgbdns-frag-f3b5b28573f6`
- Source file: [[DNS from First Principles/Code/src/transport.rs.source|src/transport.rs]]
- Lines: 78-115
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f3b5b28573f6", "codeNote": "DNS from First Principles/Code/src/transport.rs.source", "sourcePath": "src/transport.rs", "startLine": 78, "endLine": 115}
```

## Excerpt

<span id="rgbdns-frag-f3b5b28573f6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f3b5b28573f6: fn serve_tcp_connection

```rust
fn serve_tcp_connection(
    stream: &mut TcpStream,
    client: SocketAddr,
    handler: &Arc<Handler>,
    stream_handler: Option<&Arc<StreamHandler>>,
) {
    loop {
        let mut length = [0; 2];
        if stream.read_exact(&mut length).is_err() {
            return;
        }
        let mut packet = vec![0; u16::from_be_bytes(length) as usize];
        if stream.read_exact(&mut packet).is_err() {
            return;
        }
        let responses = match stream_handler
            .map(|stream_handler| stream_handler(&packet, client))
            .transpose()
        {
            Ok(Some(Some(responses))) => responses,
            Ok(_) => match handler(&packet, u16::MAX as usize, client) {
                Ok(response) => vec![response],
                Err(_) => continue,
            },
            Err(_) => return,
        };
        for response in responses {
            let Ok(response_length) = u16::try_from(response.len()) else {
                return;
            };
            if stream.write_all(&response_length.to_be_bytes()).is_err()
                || stream.write_all(&response).is_err()
            {
                return;
            }
        }
    }
}
```
