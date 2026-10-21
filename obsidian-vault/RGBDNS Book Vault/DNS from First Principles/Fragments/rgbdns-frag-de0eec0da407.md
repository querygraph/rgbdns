---
type: "code-fragment"
fragment_id: "rgbdns-frag-de0eec0da407"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "axfr"
kind: "fn"
start_line: 42
end_line: 52
---

# axfr

- Fragment ID: `rgbdns-frag-de0eec0da407`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 42-52
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-de0eec0da407", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 42, "endLine": 52}
```

## Excerpt

<span id="rgbdns-frag-de0eec0da407" class="rgbdns-fragment-target"></span>
### rgbdns-frag-de0eec0da407: fn axfr

```rust
    fn axfr(self, peer: SocketAddr, wire: &[u8], accepted: bool) {
        if self.enabled {
            write_log_line(&query_log_line_with_code(
                peer,
                wire,
                if accepted { '+' } else { '-' },
            ));
        }
    }
}

```
