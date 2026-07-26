---
type: "code-fragment"
fragment_id: "rgbdns-frag-2c97a1c68bb4"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "socket_address"
kind: "fn"
start_line: 54
end_line: 64
---

# socket_address

- Fragment ID: `rgbdns-frag-2c97a1c68bb4`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 54-64
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-2c97a1c68bb4", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-2c97a1c68bb4: fn socket_address", "sourcePath": "src/lib.rs", "startLine": 54, "endLine": 64}
```

## Excerpt

<span id="rgbdns-frag-2c97a1c68bb4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2c97a1c68bb4: fn socket_address

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
