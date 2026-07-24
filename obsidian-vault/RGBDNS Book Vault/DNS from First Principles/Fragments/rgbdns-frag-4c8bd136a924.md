---
type: "code-fragment"
fragment_id: "rgbdns-frag-4c8bd136a924"
source_path: "src/client.rs"
code_note: "DNS from First Principles/Code/src/client.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "tcp_query"
kind: "fn"
start_line: 110
end_line: 126
---

# tcp_query

- Fragment ID: `rgbdns-frag-4c8bd136a924`
- Source file: [[DNS from First Principles/Code/src/client.rs.source|src/client.rs]]
- Lines: 110-126
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4c8bd136a924", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "heading": "rgbdns-frag-4c8bd136a924: fn tcp_query", "sourcePath": "src/client.rs", "startLine": 110, "endLine": 126}
```

## Excerpt

<span id="rgbdns-frag-4c8bd136a924" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4c8bd136a924: fn tcp_query

```rust
fn tcp_query(server: SocketAddr, wire: &[u8], id: u16, question: &Question) -> Result<Message> {
    let mut stream = TcpStream::connect_timeout(&server, TIMEOUT)?;
    stream.set_read_timeout(Some(TIMEOUT))?;
    stream.set_write_timeout(Some(TIMEOUT))?;
    let wire_length =
        u16::try_from(wire.len()).map_err(|_| Error::Format("DNS query exceeds TCP framing"))?;
    let mut framed = Vec::with_capacity(wire.len() + 2);
    framed.extend(wire_length.to_be_bytes());
    framed.extend(wire);
    stream.write_all(&framed)?;
    let mut length = [0; 2];
    stream.read_exact(&mut length)?;
    let mut response = vec![0; u16::from_be_bytes(length) as usize];
    stream.read_exact(&mut response)?;
    validate(Message::decode(&response)?, id, question, true)
}

```
