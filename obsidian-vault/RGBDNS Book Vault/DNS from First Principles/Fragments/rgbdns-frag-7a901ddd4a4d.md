---
type: "code-fragment"
fragment_id: "rgbdns-frag-7a901ddd4a4d"
source_path: "src/transport.rs"
code_note: "DNS from First Principles/Code/src/transport.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "StreamHandler"
kind: "type"
start_line: 13
end_line: 14
---

# StreamHandler

- Fragment ID: `rgbdns-frag-7a901ddd4a4d`
- Source file: [[DNS from First Principles/Code/src/transport.rs.source|src/transport.rs]]
- Lines: 13-14
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7a901ddd4a4d", "codeNote": "DNS from First Principles/Code/src/transport.rs.source", "heading": "rgbdns-frag-7a901ddd4a4d: type StreamHandler", "sourcePath": "src/transport.rs", "startLine": 13, "endLine": 14}
```

## Excerpt

<span id="rgbdns-frag-7a901ddd4a4d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7a901ddd4a4d: type StreamHandler

```rust
pub(crate) type StreamHandler = dyn Fn(&[u8], IpAddr) -> Result<Option<Vec<Vec<u8>>>> + Send + Sync;

```
