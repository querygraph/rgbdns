---
type: "code-fragment"
fragment_id: "rgbdns-frag-c52452849aa8"
source_path: "src/transport.rs"
code_note: "DNS from First Principles/Code/src/transport.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "Handler"
kind: "type"
start_line: 12
end_line: 13
---

# Handler

- Fragment ID: `rgbdns-frag-c52452849aa8`
- Source file: [[DNS from First Principles/Code/src/transport.rs.source|src/transport.rs]]
- Lines: 12-13
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c52452849aa8", "codeNote": "DNS from First Principles/Code/src/transport.rs.source", "heading": "rgbdns-frag-c52452849aa8: type Handler", "sourcePath": "src/transport.rs", "startLine": 12, "endLine": 13}
```

## Excerpt

<span id="rgbdns-frag-c52452849aa8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c52452849aa8: type Handler

```rust
pub(crate) type Handler = dyn Fn(&[u8], usize, IpAddr) -> Result<Vec<u8>> + Send + Sync;

```
