---
type: "code-fragment"
fragment_id: "rgbdns-frag-690e4ae608a4"
source_path: "src/client.rs"
code_note: "DNS from First Principles/Code/src/client.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "servers"
kind: "fn"
start_line: 57
end_line: 83
---

# servers

- Fragment ID: `rgbdns-frag-690e4ae608a4`
- Source file: [[DNS from First Principles/Code/src/client.rs.source|src/client.rs]]
- Lines: 57-83
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-690e4ae608a4", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "heading": "rgbdns-frag-690e4ae608a4: fn servers", "sourcePath": "src/client.rs", "startLine": 57, "endLine": 83}
```

## Excerpt

<span id="rgbdns-frag-690e4ae608a4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-690e4ae608a4: fn servers

```rust
pub fn servers() -> Result<Vec<SocketAddr>> {
    if let Ok(value) = std::env::var("DNSCACHEIP") {
        return value
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(server_address)
            .collect();
    }
    let contents = fs::read_to_string("/etc/resolv.conf")?;
    let servers = contents
        .lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            (fields.next() == Some("nameserver"))
                .then(|| fields.next())
                .flatten()
        })
        .map(server_address)
        .collect::<Result<Vec<_>>>()?;
    if servers.is_empty() {
        Err(Error::Format("no nameserver in resolv.conf"))
    } else {
        Ok(servers)
    }
}

```
