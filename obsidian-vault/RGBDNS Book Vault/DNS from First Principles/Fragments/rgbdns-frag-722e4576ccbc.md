---
type: "code-fragment"
fragment_id: "rgbdns-frag-722e4576ccbc"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "respond_from"
kind: "fn"
start_line: 16
end_line: 24
---

# respond_from

- Fragment ID: `rgbdns-frag-722e4576ccbc`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 16-24
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-722e4576ccbc", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-722e4576ccbc: fn respond_from", "sourcePath": "src/server.rs", "startLine": 16, "endLine": 24}
```

## Excerpt

<span id="rgbdns-frag-722e4576ccbc" class="rgbdns-fragment-target"></span>
### rgbdns-frag-722e4576ccbc: fn respond_from

```rust
pub fn respond_from(
    zone: &Zone,
    wire: &[u8],
    transport_limit: usize,
    client: IpAddr,
) -> Result<Vec<u8>> {
    respond_over_transport(zone, wire, transport_limit, true, Some(client))
}

```
