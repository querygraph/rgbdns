---
type: "code-fragment"
fragment_id: "rgbdns-frag-9702afa51684"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "query_log_line"
kind: "fn"
start_line: 67
end_line: 85
---

# query_log_line

- Fragment ID: `rgbdns-frag-9702afa51684`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 67-85
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9702afa51684", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 67, "endLine": 85}
```

## Excerpt

<span id="rgbdns-frag-9702afa51684" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9702afa51684: fn query_log_line

```rust
fn query_log_line(peer: SocketAddr, wire: &[u8], response: Option<&[u8]>) -> String {
    let Ok(query) = Message::decode(wire) else {
        return malformed_log_line(peer);
    };
    if query.questions.len() != 1 {
        return malformed_log_line(peer);
    }
    let code = if query.flags & 0x8000 != 0 || query.flags & 0x7800 != 0 {
        'I'
    } else if query.questions[0].qclass != 1 && query.questions[0].qclass != 255 {
        'C'
    } else if response.is_some_and(|wire| wire.get(3).is_some_and(|flags| flags & 0x0f == 5)) {
        '-'
    } else {
        '+'
    };
    query_log_line_for_query(peer, &query, code)
}

```
