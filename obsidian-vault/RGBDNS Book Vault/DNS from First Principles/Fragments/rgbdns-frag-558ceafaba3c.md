---
type: "code-fragment"
fragment_id: "rgbdns-frag-558ceafaba3c"
source_path: "src/transport.rs"
code_note: "DNS from First Principles/Code/src/transport.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "StreamHandler"
kind: "type"
start_line: 13
end_line: 15
---

# StreamHandler

- Fragment ID: `rgbdns-frag-558ceafaba3c`
- Source file: [[DNS from First Principles/Code/src/transport.rs.source|src/transport.rs]]
- Lines: 13-15
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-558ceafaba3c", "codeNote": "DNS from First Principles/Code/src/transport.rs.source", "sourcePath": "src/transport.rs", "startLine": 13, "endLine": 15}
```

## Excerpt

<span id="rgbdns-frag-558ceafaba3c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-558ceafaba3c: type StreamHandler

```rust
pub(crate) type StreamHandler =
    dyn Fn(&[u8], SocketAddr) -> Result<Option<Vec<Vec<u8>>>> + Send + Sync;

```
