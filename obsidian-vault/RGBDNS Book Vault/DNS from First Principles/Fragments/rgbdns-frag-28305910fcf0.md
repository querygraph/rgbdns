---
type: "code-fragment"
fragment_id: "rgbdns-frag-28305910fcf0"
source_path: "src/bin/pickdns.rs"
code_note: "DNS from First Principles/Code/src/bin/pickdns.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "pickdns"
symbol: "run"
kind: "fn"
start_line: 11
end_line: 24
---

# run

- Fragment ID: `rgbdns-frag-28305910fcf0`
- Source file: [[DNS from First Principles/Code/src/bin/pickdns.rs.source|src/bin/pickdns.rs]]
- Lines: 11-24
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/pickdns|pickdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-28305910fcf0", "codeNote": "DNS from First Principles/Code/src/bin/pickdns.rs.source", "sourcePath": "src/bin/pickdns.rs", "startLine": 11, "endLine": 24}
```

## Excerpt

<span id="rgbdns-frag-28305910fcf0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-28305910fcf0: fn run

```rust
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::env::var("DATA").unwrap_or_else(|_| "data.cdb".into());
    let database = Arc::new(Database::from_file(data)?);
    let handler = Arc::new(
        move |wire: &[u8], limit: usize, client: std::net::SocketAddr| {
            database.respond(wire, limit, client.ip())
        },
    );
    let ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "53".into());
    let address = rgbdns::socket_address(&ip, &port)?;
    special::serve(&address.to_string(), handler)?;
    Ok(())
}
```
