---
type: "code-fragment"
fragment_id: "rgbdns-frag-c4ff7d3f54c7"
source_path: "src/client.rs"
code_note: "DNS from First Principles/Code/src/client.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "server_address"
kind: "fn"
start_line: 84
end_line: 93
---

# server_address

- Fragment ID: `rgbdns-frag-c4ff7d3f54c7`
- Source file: [[DNS from First Principles/Code/src/client.rs.source|src/client.rs]]
- Lines: 84-93
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c4ff7d3f54c7", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "heading": "rgbdns-frag-c4ff7d3f54c7: fn server_address", "sourcePath": "src/client.rs", "startLine": 84, "endLine": 93}
```

## Excerpt

<span id="rgbdns-frag-c4ff7d3f54c7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c4ff7d3f54c7: fn server_address

```rust
pub fn server_address(value: &str) -> Result<SocketAddr> {
    if let Ok(address) = value.parse() {
        return Ok(address);
    }
    value
        .parse::<IpAddr>()
        .map(|address| SocketAddr::new(address, 53))
        .map_err(|_| Error::Format("invalid DNS server address"))
}

```
