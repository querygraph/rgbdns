---
type: "code-fragment"
fragment_id: "rgbdns-frag-7922cf070768"
source_path: "src/setuidgid.rs"
code_note: "DNS from First Principles/Code/src/setuidgid.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "Identity"
kind: "struct"
start_line: 9
end_line: 14
---

# Identity

- Fragment ID: `rgbdns-frag-7922cf070768`
- Source file: [[DNS from First Principles/Code/src/setuidgid.rs.source|src/setuidgid.rs]]
- Lines: 9-14
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7922cf070768", "codeNote": "DNS from First Principles/Code/src/setuidgid.rs.source", "heading": "rgbdns-frag-7922cf070768: struct Identity", "sourcePath": "src/setuidgid.rs", "startLine": 9, "endLine": 14}
```

## Excerpt

<span id="rgbdns-frag-7922cf070768" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7922cf070768: struct Identity

```rust
pub struct Identity {
    pub name: String,
    pub uid: Uid,
    pub gid: Gid,
}

```
