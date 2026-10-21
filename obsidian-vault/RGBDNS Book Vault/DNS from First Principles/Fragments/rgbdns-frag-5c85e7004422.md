---
type: "code-fragment"
fragment_id: "rgbdns-frag-5c85e7004422"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "error_response"
kind: "fn"
start_line: 427
end_line: 438
---

# error_response

- Fragment ID: `rgbdns-frag-5c85e7004422`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 427-438
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5c85e7004422", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 427, "endLine": 438}
```

## Excerpt

<span id="rgbdns-frag-5c85e7004422" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5c85e7004422: fn error_response

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
