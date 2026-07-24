---
type: "code-fragment"
fragment_id: "rgbdns-frag-813ff75d332f"
source_path: "src/bin/walldns.rs"
code_note: "DNS from First Principles/Code/src/bin/walldns.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "walldns"
symbol: "run"
kind: "fn"
start_line: 11
end_line: 20
---

# run

- Fragment ID: `rgbdns-frag-813ff75d332f`
- Source file: [[DNS from First Principles/Code/src/bin/walldns.rs.source|src/bin/walldns.rs]]
- Lines: 11-20
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/walldns|walldns]]

```rgbdns-fragment
{"id": "rgbdns-frag-813ff75d332f", "codeNote": "DNS from First Principles/Code/src/bin/walldns.rs.source", "heading": "rgbdns-frag-813ff75d332f: fn run", "sourcePath": "src/bin/walldns.rs", "startLine": 11, "endLine": 20}
```

## Excerpt

<span id="rgbdns-frag-813ff75d332f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-813ff75d332f: fn run

```rust
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "53".into());
    let address = rgbdns::socket_address(&ip, &port)?;
    special::serve(
        &address.to_string(),
        Arc::new(|wire: &[u8], limit: usize, _| wall::respond(wire, limit)),
    )?;
    Ok(())
}
```
