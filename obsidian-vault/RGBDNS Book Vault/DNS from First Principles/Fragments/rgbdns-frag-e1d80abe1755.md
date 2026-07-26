---
type: "code-fragment"
fragment_id: "rgbdns-frag-e1d80abe1755"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "Aname"
kind: "struct"
start_line: 25
end_line: 30
---

# Aname

- Fragment ID: `rgbdns-frag-e1d80abe1755`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 25-30
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e1d80abe1755", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-e1d80abe1755: struct Aname", "sourcePath": "src/zone.rs", "startLine": 25, "endLine": 30}
```

## Excerpt

<span id="rgbdns-frag-e1d80abe1755" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e1d80abe1755: struct Aname

```rust
pub(crate) struct Aname {
    pub target: Name,
    pub ttl: u32,
}

#[derive(Clone, Copy, Debug, Default)]
```
