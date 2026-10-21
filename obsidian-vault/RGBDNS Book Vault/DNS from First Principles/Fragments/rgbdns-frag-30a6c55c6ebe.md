---
type: "code-fragment"
fragment_id: "rgbdns-frag-30a6c55c6ebe"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "from_system"
kind: "fn"
start_line: 37
end_line: 40
---

# from_system

- Fragment ID: `rgbdns-frag-30a6c55c6ebe`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 37-40
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-30a6c55c6ebe", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "sourcePath": "src/aname.rs", "startLine": 37, "endLine": 40}
```

## Excerpt

<span id="rgbdns-frag-30a6c55c6ebe" class="rgbdns-fragment-target"></span>
### rgbdns-frag-30a6c55c6ebe: fn from_system

```rust
    pub(crate) fn from_system() -> Result<Self> {
        Ok(Self::new(client::servers()?))
    }

```
