---
type: "code-fragment"
fragment_id: "rgbdns-frag-6acd0c6e28f0"
source_path: "src/transport.rs"
code_note: "DNS from First Principles/Code/src/transport.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "Handler"
kind: "type"
start_line: 12
end_line: 12
---

# Handler

- Fragment ID: `rgbdns-frag-6acd0c6e28f0`
- Source file: [[DNS from First Principles/Code/src/transport.rs.source|src/transport.rs]]
- Lines: 12-12
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-6acd0c6e28f0", "codeNote": "DNS from First Principles/Code/src/transport.rs.source", "heading": "rgbdns-frag-6acd0c6e28f0: type Handler", "sourcePath": "src/transport.rs", "startLine": 12, "endLine": 12}
```

## Excerpt

<span id="rgbdns-frag-6acd0c6e28f0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6acd0c6e28f0: type Handler

```rust
pub(crate) type Handler = dyn Fn(&[u8], usize, IpAddr) -> Result<Vec<u8>> + Send + Sync;
```
