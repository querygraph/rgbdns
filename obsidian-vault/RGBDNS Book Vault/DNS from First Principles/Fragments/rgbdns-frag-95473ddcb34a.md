---
type: "code-fragment"
fragment_id: "rgbdns-frag-95473ddcb34a"
source_path: "src/transport.rs"
code_note: "DNS from First Principles/Code/src/transport.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "TCP_TIMEOUT"
kind: "const"
start_line: 15
end_line: 16
---

# TCP_TIMEOUT

- Fragment ID: `rgbdns-frag-95473ddcb34a`
- Source file: [[DNS from First Principles/Code/src/transport.rs.source|src/transport.rs]]
- Lines: 15-16
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-95473ddcb34a", "codeNote": "DNS from First Principles/Code/src/transport.rs.source", "heading": "rgbdns-frag-95473ddcb34a: const TCP_TIMEOUT", "sourcePath": "src/transport.rs", "startLine": 15, "endLine": 16}
```

## Excerpt

<span id="rgbdns-frag-95473ddcb34a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-95473ddcb34a: const TCP_TIMEOUT

```rust
const TCP_TIMEOUT: Duration = Duration::from_secs(10);

```
