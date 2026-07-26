---
type: "code-fragment"
fragment_id: "rgbdns-frag-fac10ff0d148"
source_path: "src/transport.rs"
code_note: "DNS from First Principles/Code/src/transport.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "serve_tcp_connection"
kind: "fn"
start_line: 77
end_line: 114
---

# serve_tcp_connection

- Fragment ID: `rgbdns-frag-fac10ff0d148`
- Source file: [[DNS from First Principles/Code/src/transport.rs.source|src/transport.rs]]
- Lines: 77-114
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-fac10ff0d148", "codeNote": "DNS from First Principles/Code/src/transport.rs.source", "heading": "rgbdns-frag-fac10ff0d148: fn serve_tcp_connection", "sourcePath": "src/transport.rs", "startLine": 77, "endLine": 114}
```

## Excerpt

<span id="rgbdns-frag-fac10ff0d148" class="rgbdns-fragment-target"></span>
### rgbdns-frag-fac10ff0d148: fn serve_tcp_connection

```rust
fn serve_tcp_connection(
    stream: &mut TcpStream,
    client: IpAddr,
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
