---
type: "code-fragment"
fragment_id: "rgbdns-frag-abd7554f2908"
source_path: "src/tai64.rs"
code_note: "DNS from First Principles/Code/src/tai64.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "Timestamp"
kind: "struct"
start_line: 44
end_line: 48
---

# Timestamp

- Fragment ID: `rgbdns-frag-abd7554f2908`
- Source file: [[DNS from First Principles/Code/src/tai64.rs.source|src/tai64.rs]]
- Lines: 44-48
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-abd7554f2908", "codeNote": "DNS from First Principles/Code/src/tai64.rs.source", "heading": "rgbdns-frag-abd7554f2908: struct Timestamp", "sourcePath": "src/tai64.rs", "startLine": 44, "endLine": 48}
```

## Excerpt

<span id="rgbdns-frag-abd7554f2908" class="rgbdns-fragment-target"></span>
### rgbdns-frag-abd7554f2908: struct Timestamp

```rust
pub struct Timestamp {
    pub unix_seconds: i64,
    pub nanoseconds: u32,
}

```
