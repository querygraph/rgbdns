---
type: "code-fragment"
fragment_id: "rgbdns-frag-0151d5c4fe7b"
source_path: "src/bin/rbldns.rs"
code_note: "DNS from First Principles/Code/src/bin/rbldns.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rbldns"
symbol: "run"
kind: "fn"
start_line: 11
end_line: 24
---

# run

- Fragment ID: `rgbdns-frag-0151d5c4fe7b`
- Source file: [[DNS from First Principles/Code/src/bin/rbldns.rs.source|src/bin/rbldns.rs]]
- Lines: 11-24
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rbldns|rbldns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0151d5c4fe7b", "codeNote": "DNS from First Principles/Code/src/bin/rbldns.rs.source", "heading": "rgbdns-frag-0151d5c4fe7b: fn run", "sourcePath": "src/bin/rbldns.rs", "startLine": 11, "endLine": 24}
```

## Excerpt

<span id="rgbdns-frag-0151d5c4fe7b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0151d5c4fe7b: fn run

```rust
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::env::var("DATA").unwrap_or_else(|_| "data.cdb".into());
    let base: Name = std::env::var("BASE")
        .map_err(|_| "BASE is required")?
        .parse()?;
    let database = Arc::new(Database::from_file(data)?);
    let handler =
        Arc::new(move |wire: &[u8], limit: usize, _| database.respond(&base, wire, limit));
    let ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "53".into());
    let address = rgbdns::socket_address(&ip, &port)?;
    special::serve(&address.to_string(), handler)?;
    Ok(())
}
```
