---
type: "code-fragment"
fragment_id: "rgbdns-frag-d7ec343834ce"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "query_log_line_for_query"
kind: "fn"
start_line: 99
end_line: 116
---

# query_log_line_for_query

- Fragment ID: `rgbdns-frag-d7ec343834ce`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 99-116
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d7ec343834ce", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 99, "endLine": 116}
```

## Excerpt

<span id="rgbdns-frag-d7ec343834ce" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d7ec343834ce: fn query_log_line_for_query

```rust
fn query_log_line_for_query(peer: SocketAddr, query: &Message, code: char) -> String {
    let question = &query.questions[0];
    let display_name = question.name.to_string();
    let display_name = if display_name == "." {
        display_name.as_str()
    } else {
        display_name.strip_suffix('.').unwrap_or(&display_name)
    };
    format!(
        "{}:{:04x}:{:04x} {code} {:04x} {}",
        log_ip(peer.ip()),
        peer.port(),
        query.id,
        question.qtype.code(),
        display_name
    )
}

```
