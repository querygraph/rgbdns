---
type: "code-fragment"
fragment_id: "rgbdns-frag-e264d7a03b5d"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "log_ip"
kind: "fn"
start_line: 121
end_line: 135
---

# log_ip

- Fragment ID: `rgbdns-frag-e264d7a03b5d`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 121-135
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e264d7a03b5d", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 121, "endLine": 135}
```

## Excerpt

<span id="rgbdns-frag-e264d7a03b5d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e264d7a03b5d: fn log_ip

```rust
fn log_ip(address: IpAddr) -> String {
    match address {
        IpAddr::V4(address) => address
            .octets()
            .iter()
            .map(|octet| format!("{octet:02x}"))
            .collect(),
        IpAddr::V6(address) => address
            .octets()
            .iter()
            .map(|octet| format!("{octet:02x}"))
            .collect(),
    }
}

```
