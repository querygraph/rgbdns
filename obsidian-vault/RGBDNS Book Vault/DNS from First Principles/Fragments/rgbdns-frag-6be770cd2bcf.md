---
type: "code-fragment"
fragment_id: "rgbdns-frag-6be770cd2bcf"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "ParsedTsig"
kind: "struct"
start_line: 238
end_line: 248
---

# ParsedTsig

- Fragment ID: `rgbdns-frag-6be770cd2bcf`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 238-248
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-6be770cd2bcf", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 238, "endLine": 248}
```

## Excerpt

<span id="rgbdns-frag-6be770cd2bcf" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6be770cd2bcf: struct ParsedTsig

```rust
struct ParsedTsig {
    key_name: Name,
    algorithm: Name,
    time_signed: u64,
    fudge: u16,
    mac: Vec<u8>,
    original_id: u16,
    error: u16,
    other: Vec<u8>,
}

```
