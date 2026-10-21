---
type: "code-fragment"
fragment_id: "rgbdns-frag-f2ef2cf7d6dd"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "serve"
kind: "fn"
start_line: 23
end_line: 30
---

# serve

- Fragment ID: `rgbdns-frag-f2ef2cf7d6dd`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 23-30
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f2ef2cf7d6dd", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 23, "endLine": 30}
```

## Excerpt

<span id="rgbdns-frag-f2ef2cf7d6dd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f2ef2cf7d6dd: fn serve

```rust
pub fn serve(zone: Zone, address: &str, allowed: Vec<IpNet>) -> Result<()> {
    serve_listener(
        Arc::new(zone),
        TcpListener::bind(address)?,
        Arc::new(allowed),
    )
}

```
