---
type: "code-fragment"
fragment_id: "rgbdns-frag-0ccf5c904072"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "query_log_line_with_code"
kind: "fn"
start_line: 86
end_line: 98
---

# query_log_line_with_code

- Fragment ID: `rgbdns-frag-0ccf5c904072`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 86-98
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0ccf5c904072", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 86, "endLine": 98}
```

## Excerpt

<span id="rgbdns-frag-0ccf5c904072" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0ccf5c904072: fn query_log_line_with_code

```rust
fn query_log_line_with_code(peer: SocketAddr, wire: &[u8], code: char) -> String {
    Message::decode(wire).map_or_else(
        |_| malformed_log_line(peer),
        |query| {
            if query.questions.len() == 1 {
                query_log_line_for_query(peer, &query, code)
            } else {
                malformed_log_line(peer)
            }
        },
    )
}

```
