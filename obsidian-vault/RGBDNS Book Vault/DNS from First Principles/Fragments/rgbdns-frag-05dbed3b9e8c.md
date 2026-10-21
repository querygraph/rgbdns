---
type: "code-fragment"
fragment_id: "rgbdns-frag-05dbed3b9e8c"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "request"
kind: "fn"
start_line: 36
end_line: 41
---

# request

- Fragment ID: `rgbdns-frag-05dbed3b9e8c`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 36-41
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-05dbed3b9e8c", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 36, "endLine": 41}
```

## Excerpt

<span id="rgbdns-frag-05dbed3b9e8c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-05dbed3b9e8c: fn request

```rust
    fn request(self, peer: SocketAddr, wire: &[u8], response: Option<&[u8]>) {
        if self.enabled {
            write_log_line(&query_log_line(peer, wire, response));
        }
    }

```
