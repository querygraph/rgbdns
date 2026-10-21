---
type: "code-fragment"
fragment_id: "rgbdns-frag-dafb0dd4284e"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "new"
kind: "fn"
start_line: 903
end_line: 908
---

# new

- Fragment ID: `rgbdns-frag-dafb0dd4284e`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 903-908
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-dafb0dd4284e", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 903, "endLine": 908}
```

## Excerpt

<span id="rgbdns-frag-dafb0dd4284e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-dafb0dd4284e: fn new

```rust
    fn new(wire: &'a [u8]) -> Result<Self> {
        if wire.len() < 12 && !wire.is_empty() {
            return Err(Error::Format("short UPDATE message"));
        }
        Ok(Self { wire, position: 0 })
    }
```
