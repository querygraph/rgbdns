---
type: "code-fragment"
fragment_id: "rgbdns-frag-583cdf24733b"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "from_system"
kind: "fn"
start_line: 33
end_line: 36
---

# from_system

- Fragment ID: `rgbdns-frag-583cdf24733b`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 33-36
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-583cdf24733b", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "heading": "rgbdns-frag-583cdf24733b: fn from_system", "sourcePath": "src/aname.rs", "startLine": 33, "endLine": 36}
```

## Excerpt

<span id="rgbdns-frag-583cdf24733b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-583cdf24733b: fn from_system

```rust
    pub(crate) fn from_system() -> Result<Self> {
        Ok(Self::new(client::servers()?))
    }

```
