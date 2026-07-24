---
type: "code-fragment"
fragment_id: "rgbdns-frag-9a2021bd9440"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "serve_listener"
kind: "fn"
start_line: 28
end_line: 61
---

# serve_listener

- Fragment ID: `rgbdns-frag-9a2021bd9440`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 28-61
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9a2021bd9440", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-9a2021bd9440: fn serve_listener", "sourcePath": "src/axfr.rs", "startLine": 28, "endLine": 61}
```

## Excerpt

<span id="rgbdns-frag-9a2021bd9440" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9a2021bd9440: fn serve_listener

```rust
pub fn serve_listener(
    zone: Arc<Zone>,
    listener: TcpListener,
    allowed: Arc<Vec<IpNet>>,
) -> Result<()> {
    let mut workers = Vec::new();
    for _ in 0..16 {
        let zone = zone.clone();
        let allowed = allowed.clone();
        let listener = listener.try_clone()?;
        workers.push(thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else {
                    continue;
                };
                let peer = match stream.peer_addr() {
                    Ok(peer) => peer,
                    Err(_) => continue,
                };
                if !allowed.iter().any(|network| network.contains(&peer.ip())) {
                    continue;
                }
                let _ = stream.set_read_timeout(Some(Duration::from_secs(30)));
                let _ = stream.set_write_timeout(Some(Duration::from_secs(30)));
                let _ = serve_connection(&zone, &mut stream);
            }
        }));
    }
    for worker in workers {
        let _ = worker.join();
    }
    Ok(())
}

```
