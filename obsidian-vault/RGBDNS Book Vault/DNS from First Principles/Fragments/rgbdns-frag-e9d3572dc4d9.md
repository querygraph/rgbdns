---
type: "code-fragment"
fragment_id: "rgbdns-frag-e9d3572dc4d9"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "socket_address"
kind: "fn"
start_line: 56
end_line: 66
---

# socket_address

- Fragment ID: `rgbdns-frag-e9d3572dc4d9`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 56-66
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e9d3572dc4d9", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "sourcePath": "src/lib.rs", "startLine": 56, "endLine": 66}
```

## Excerpt

<span id="rgbdns-frag-e9d3572dc4d9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e9d3572dc4d9: fn socket_address

```rust
pub fn socket_address(ip: &str, port: &str) -> Result<std::net::SocketAddr> {
    let ip = ip
        .parse::<std::net::IpAddr>()
        .map_err(|_| Error::Format("invalid IP address"))?;
    let port = port
        .parse::<u16>()
        .map_err(|_| Error::Format("invalid port"))?;
    Ok(std::net::SocketAddr::new(ip, port))
}

#[cfg(test)]
```
