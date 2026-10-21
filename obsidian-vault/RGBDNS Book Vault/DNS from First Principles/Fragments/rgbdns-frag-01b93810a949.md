---
type: "code-fragment"
fragment_id: "rgbdns-frag-01b93810a949"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "constant_time_eq"
kind: "fn"
start_line: 1138
end_line: 1147
---

# constant_time_eq

- Fragment ID: `rgbdns-frag-01b93810a949`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 1138-1147
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-01b93810a949", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 1138, "endLine": 1147}
```

## Excerpt

<span id="rgbdns-frag-01b93810a949" class="rgbdns-fragment-target"></span>
### rgbdns-frag-01b93810a949: fn constant_time_eq

```rust
fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    left.iter()
        .zip(right)
        .fold(0u8, |difference, (a, b)| difference | (a ^ b))
        == 0
}

```
