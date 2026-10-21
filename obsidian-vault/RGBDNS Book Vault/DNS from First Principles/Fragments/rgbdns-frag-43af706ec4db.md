---
type: "code-fragment"
fragment_id: "rgbdns-frag-43af706ec4db"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "INFLIGHT_WAIT"
kind: "const"
start_line: 15
end_line: 17
---

# INFLIGHT_WAIT

- Fragment ID: `rgbdns-frag-43af706ec4db`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 15-17
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-43af706ec4db", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "sourcePath": "src/aname.rs", "startLine": 15, "endLine": 17}
```

## Excerpt

<span id="rgbdns-frag-43af706ec4db" class="rgbdns-fragment-target"></span>
### rgbdns-frag-43af706ec4db: const INFLIGHT_WAIT

```rust
const INFLIGHT_WAIT: Duration = Duration::from_secs(30);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
```
