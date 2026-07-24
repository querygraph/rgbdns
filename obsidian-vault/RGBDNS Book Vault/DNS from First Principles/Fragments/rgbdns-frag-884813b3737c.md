---
type: "code-fragment"
fragment_id: "rgbdns-frag-884813b3737c"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "serve"
kind: "fn"
start_line: 20
end_line: 27
---

# serve

- Fragment ID: `rgbdns-frag-884813b3737c`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 20-27
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-884813b3737c", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-884813b3737c: fn serve", "sourcePath": "src/axfr.rs", "startLine": 20, "endLine": 27}
```

## Excerpt

<span id="rgbdns-frag-884813b3737c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-884813b3737c: fn serve

```rust
pub fn serve(zone: Zone, address: &str, allowed: Vec<IpNet>) -> Result<()> {
    serve_listener(
        Arc::new(zone),
        TcpListener::bind(address)?,
        Arc::new(allowed),
    )
}

```
