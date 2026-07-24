---
type: "code-fragment"
fragment_id: "rgbdns-frag-079271500f1a"
source_path: "src/pick.rs"
code_note: "DNS from First Principles/Code/src/pick.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "Database"
kind: "struct"
start_line: 12
end_line: 16
---

# Database

- Fragment ID: `rgbdns-frag-079271500f1a`
- Source file: [[DNS from First Principles/Code/src/pick.rs.source|src/pick.rs]]
- Lines: 12-16
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-079271500f1a", "codeNote": "DNS from First Principles/Code/src/pick.rs.source", "heading": "rgbdns-frag-079271500f1a: struct Database", "sourcePath": "src/pick.rs", "startLine": 12, "endLine": 16}
```

## Excerpt

<span id="rgbdns-frag-079271500f1a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-079271500f1a: struct Database

```rust
pub struct Database {
    addresses: BTreeMap<([u8; 2], Name), Vec<Ipv4Addr>>,
    locations: Vec<(Vec<u8>, [u8; 2])>,
}

```
