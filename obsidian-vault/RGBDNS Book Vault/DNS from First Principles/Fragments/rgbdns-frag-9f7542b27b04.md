---
type: "code-fragment"
fragment_id: "rgbdns-frag-9f7542b27b04"
source_path: "src/rbl.rs"
code_note: "DNS from First Principles/Code/src/rbl.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "Database"
kind: "struct"
start_line: 7
end_line: 11
---

# Database

- Fragment ID: `rgbdns-frag-9f7542b27b04`
- Source file: [[DNS from First Principles/Code/src/rbl.rs.source|src/rbl.rs]]
- Lines: 7-11
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9f7542b27b04", "codeNote": "DNS from First Principles/Code/src/rbl.rs.source", "heading": "rgbdns-frag-9f7542b27b04: struct Database", "sourcePath": "src/rbl.rs", "startLine": 7, "endLine": 11}
```

## Excerpt

<span id="rgbdns-frag-9f7542b27b04" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9f7542b27b04: struct Database

```rust
pub struct Database {
    networks: HashSet<(u32, u8)>,
    responses: Vec<(Ipv4Addr, Vec<u8>)>,
}

```
