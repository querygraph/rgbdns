---
type: "code-fragment"
fragment_id: "rgbdns-frag-257f816a441c"
source_path: "src/transport.rs"
code_note: "DNS from First Principles/Code/src/transport.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "TCP_TIMEOUT"
kind: "const"
start_line: 17
end_line: 18
---

# TCP_TIMEOUT

- Fragment ID: `rgbdns-frag-257f816a441c`
- Source file: [[DNS from First Principles/Code/src/transport.rs.source|src/transport.rs]]
- Lines: 17-18
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-257f816a441c", "codeNote": "DNS from First Principles/Code/src/transport.rs.source", "sourcePath": "src/transport.rs", "startLine": 17, "endLine": 18}
```

## Excerpt

<span id="rgbdns-frag-257f816a441c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-257f816a441c: const TCP_TIMEOUT

```rust
const TCP_TIMEOUT: Duration = Duration::from_secs(10);

```
