---
type: "code-fragment"
fragment_id: "rgbdns-frag-3fd165496cd9"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "generate_key"
kind: "fn"
start_line: 323
end_line: 328
---

# generate_key

- Fragment ID: `rgbdns-frag-3fd165496cd9`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 323-328
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3fd165496cd9", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 323, "endLine": 328}
```

## Excerpt

<span id="rgbdns-frag-3fd165496cd9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3fd165496cd9: fn generate_key

```rust
pub fn generate_key(_zone: &Name, _path: &Path) -> Result<Policy> {
    Err(Error::Format(
        "DNSSEC key generation requires Unix file modes",
    ))
}

```
