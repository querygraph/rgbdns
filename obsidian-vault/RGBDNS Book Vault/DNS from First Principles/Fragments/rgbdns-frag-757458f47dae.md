---
type: "code-fragment"
fragment_id: "rgbdns-frag-757458f47dae"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "malformed_log_line"
kind: "fn"
start_line: 117
end_line: 120
---

# malformed_log_line

- Fragment ID: `rgbdns-frag-757458f47dae`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 117-120
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-757458f47dae", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 117, "endLine": 120}
```

## Excerpt

<span id="rgbdns-frag-757458f47dae" class="rgbdns-fragment-target"></span>
### rgbdns-frag-757458f47dae: fn malformed_log_line

```rust
fn malformed_log_line(peer: SocketAddr) -> String {
    format!("{}:{:04x}:0000 / 0000 .", log_ip(peer.ip()), peer.port())
}

```
