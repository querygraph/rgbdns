---
type: "code-fragment"
fragment_id: "rgbdns-frag-047fdcb9b19e"
source_path: "src/transport.rs"
code_note: "DNS from First Principles/Code/src/transport.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "serve_sockets"
kind: "fn"
start_line: 25
end_line: 53
---

# serve_sockets

- Fragment ID: `rgbdns-frag-047fdcb9b19e`
- Source file: [[DNS from First Principles/Code/src/transport.rs.source|src/transport.rs]]
- Lines: 25-53
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-047fdcb9b19e", "codeNote": "DNS from First Principles/Code/src/transport.rs.source", "heading": "rgbdns-frag-047fdcb9b19e: fn serve_sockets", "sourcePath": "src/transport.rs", "startLine": 25, "endLine": 53}
```

## Excerpt

<span id="rgbdns-frag-047fdcb9b19e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-047fdcb9b19e: fn serve_sockets

```rust
pub(crate) fn serve_sockets(udp: UdpSocket, tcp: TcpListener, handler: Arc<Handler>) -> Result<()> {
    let udp_handler = handler.clone();
    thread::spawn(move || serve_udp(udp, &udp_handler));

    let mut workers = Vec::with_capacity(TCP_WORKERS);
    for _ in 0..TCP_WORKERS {
        let handler = handler.clone();
        let listener = tcp.try_clone()?;
        workers.push(thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else {
                    continue;
                };
                let client = match stream.peer_addr() {
                    Ok(peer) => peer.ip(),
                    Err(_) => continue,
                };
                let _ = stream.set_read_timeout(Some(TCP_TIMEOUT));
                let _ = stream.set_write_timeout(Some(TCP_TIMEOUT));
                serve_tcp_connection(&mut stream, client, &handler);
            }
        }));
    }
    for worker in workers {
        let _ = worker.join();
    }
    Ok(())
}

```
