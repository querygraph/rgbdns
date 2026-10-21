---
type: "code-fragment"
fragment_id: "rgbdns-frag-d5800cc1d314"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "create_handle"
kind: "fn"
start_line: 93
end_line: 96
---

# create_handle

- Fragment ID: `rgbdns-frag-d5800cc1d314`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 93-96
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d5800cc1d314", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 93, "endLine": 96}
```

## Excerpt

<span id="rgbdns-frag-d5800cc1d314" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d5800cc1d314: fn create_handle

```rust
    fn create_handle(&self) -> Self::Handle {
        self.0.create_handle()
    }

```
