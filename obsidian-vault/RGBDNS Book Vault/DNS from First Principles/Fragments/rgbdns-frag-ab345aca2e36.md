---
type: "code-fragment"
fragment_id: "rgbdns-frag-ab345aca2e36"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "new"
kind: "fn"
start_line: 37
end_line: 43
---

# new

- Fragment ID: `rgbdns-frag-ab345aca2e36`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 37-43
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ab345aca2e36", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "heading": "rgbdns-frag-ab345aca2e36: fn new", "sourcePath": "src/aname.rs", "startLine": 37, "endLine": 43}
```

## Excerpt

<span id="rgbdns-frag-ab345aca2e36" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ab345aca2e36: fn new

```rust
    pub(crate) fn new(servers: Vec<SocketAddr>) -> Self {
        Self {
            servers,
            cache: Mutex::new(HashMap::new()),
        }
    }

```
