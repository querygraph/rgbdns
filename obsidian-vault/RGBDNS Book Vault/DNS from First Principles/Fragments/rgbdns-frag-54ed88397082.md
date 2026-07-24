---
type: "code-fragment"
fragment_id: "rgbdns-frag-54ed88397082"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "socket_address"
kind: "fn"
start_line: 53
end_line: 63
---

# socket_address

- Fragment ID: `rgbdns-frag-54ed88397082`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 53-63
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-54ed88397082", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-54ed88397082: fn socket_address", "sourcePath": "src/lib.rs", "startLine": 53, "endLine": 63}
```

## Excerpt

<span id="rgbdns-frag-54ed88397082" class="rgbdns-fragment-target"></span>
### rgbdns-frag-54ed88397082: fn socket_address

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
