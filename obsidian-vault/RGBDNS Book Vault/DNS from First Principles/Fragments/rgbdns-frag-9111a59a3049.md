---
type: "code-fragment"
fragment_id: "rgbdns-frag-9111a59a3049"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "respond"
kind: "fn"
start_line: 12
end_line: 15
---

# respond

- Fragment ID: `rgbdns-frag-9111a59a3049`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 12-15
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9111a59a3049", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-9111a59a3049: fn respond", "sourcePath": "src/server.rs", "startLine": 12, "endLine": 15}
```

## Excerpt

<span id="rgbdns-frag-9111a59a3049" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9111a59a3049: fn respond

```rust
pub fn respond(zone: &Zone, wire: &[u8], transport_limit: usize) -> Result<Vec<u8>> {
    respond_over_transport(zone, wire, transport_limit, true, None)
}

```
