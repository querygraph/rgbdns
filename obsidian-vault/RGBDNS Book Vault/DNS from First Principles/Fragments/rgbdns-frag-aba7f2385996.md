---
type: "code-fragment"
fragment_id: "rgbdns-frag-aba7f2385996"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "error_response"
kind: "fn"
start_line: 227
end_line: 238
---

# error_response

- Fragment ID: `rgbdns-frag-aba7f2385996`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 227-238
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-aba7f2385996", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-aba7f2385996: fn error_response", "sourcePath": "src/server.rs", "startLine": 227, "endLine": 238}
```

## Excerpt

<span id="rgbdns-frag-aba7f2385996" class="rgbdns-fragment-target"></span>
### rgbdns-frag-aba7f2385996: fn error_response

```rust
fn error_response(query: &[u8], rcode: u16) -> Result<Vec<u8>> {
    if query.len() < 4 {
        return Err(Error::Format("short DNS query"));
    }
    Message {
        id: u16::from_be_bytes([query[0], query[1]]),
        flags: 0x8000 | (u16::from_be_bytes([query[2], query[3]]) & 0x7900) | rcode,
        ..Default::default()
    }
    .encode()
}

```
