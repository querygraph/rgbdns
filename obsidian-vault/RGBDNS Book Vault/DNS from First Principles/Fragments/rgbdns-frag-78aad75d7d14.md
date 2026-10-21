---
type: "code-fragment"
fragment_id: "rgbdns-frag-78aad75d7d14"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "Policy"
kind: "struct"
start_line: 121
end_line: 129
---

# Policy

- Fragment ID: `rgbdns-frag-78aad75d7d14`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 121-129
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-78aad75d7d14", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 121, "endLine": 129}
```

## Excerpt

<span id="rgbdns-frag-78aad75d7d14" class="rgbdns-fragment-target"></span>
### rgbdns-frag-78aad75d7d14: struct Policy

```rust
pub struct Policy {
    pub zone: Name,
    pub key_file: PathBuf,
    pub validity: u64,
    pub refresh: u64,
    pub inception_skew: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
```
