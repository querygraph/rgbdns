---
type: "code-fragment"
fragment_id: "rgbdns-frag-8adf36f03d5d"
source_path: "src/client.rs"
code_note: "DNS from First Principles/Code/src/client.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "query"
kind: "fn"
start_line: 17
end_line: 56
---

# query

- Fragment ID: `rgbdns-frag-8adf36f03d5d`
- Source file: [[DNS from First Principles/Code/src/client.rs.source|src/client.rs]]
- Lines: 17-56
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-8adf36f03d5d", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "heading": "rgbdns-frag-8adf36f03d5d: fn query", "sourcePath": "src/client.rs", "startLine": 17, "endLine": 56}
```

## Excerpt

<span id="rgbdns-frag-8adf36f03d5d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8adf36f03d5d: fn query

```rust
pub fn query(
    name: Name,
    record_type: RecordType,
    recursion_desired: bool,
    servers: &[SocketAddr],
) -> Result<Message> {
    let id = random_id()?;
    let question = Question {
        name,
        qtype: record_type,
        qclass: 1,
    };
    let wire = Message {
        id,
        flags: if recursion_desired { 0x0100 } else { 0 },
        questions: vec![question.clone()],
        ..Default::default()
    }
    .encode()?;
    let mut last_error = None;
    for server in servers
        .iter()
        .copied()
        .cycle()
        .take(servers.len().max(1) * 2)
    {
        match udp_query(server, &wire, id, &question) {
            Ok(response) if response.flags & 0x0200 != 0 => {
                match tcp_query(server, &wire, id, &question) {
                    Ok(response) => return Ok(response),
                    Err(error) => last_error = Some(error),
                }
            }
            Ok(response) => return Ok(response),
            Err(error) => last_error = Some(error),
        }
    }
    Err(last_error.unwrap_or(Error::Format("no recursive DNS servers configured")))
}

```
