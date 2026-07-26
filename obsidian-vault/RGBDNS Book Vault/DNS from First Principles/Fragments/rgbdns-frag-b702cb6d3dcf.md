---
type: "code-fragment"
fragment_id: "rgbdns-frag-b702cb6d3dcf"
source_path: "src/transport.rs"
code_note: "DNS from First Principles/Code/src/transport.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "TCP_TIMEOUT"
kind: "const"
start_line: 16
end_line: 17
---

# TCP_TIMEOUT

- Fragment ID: `rgbdns-frag-b702cb6d3dcf`
- Source file: [[DNS from First Principles/Code/src/transport.rs.source|src/transport.rs]]
- Lines: 16-17
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b702cb6d3dcf", "codeNote": "DNS from First Principles/Code/src/transport.rs.source", "heading": "rgbdns-frag-b702cb6d3dcf: const TCP_TIMEOUT", "sourcePath": "src/transport.rs", "startLine": 16, "endLine": 17}
```

## Excerpt

<span id="rgbdns-frag-b702cb6d3dcf" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b702cb6d3dcf: const TCP_TIMEOUT

```rust
const TCP_TIMEOUT: Duration = Duration::from_secs(10);

```
