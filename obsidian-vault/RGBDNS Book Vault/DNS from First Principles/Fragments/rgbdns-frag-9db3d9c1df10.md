---
type: "code-fragment"
fragment_id: "rgbdns-frag-9db3d9c1df10"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "random_id"
kind: "fn"
start_line: 332
end_line: 338
---

# random_id

- Fragment ID: `rgbdns-frag-9db3d9c1df10`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 332-338
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9db3d9c1df10", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 332, "endLine": 338}
```

## Excerpt

<span id="rgbdns-frag-9db3d9c1df10" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9db3d9c1df10: fn random_id

```rust
fn random_id() -> Result<u16> {
    let mut bytes = [0; 2];
    getrandom::fill(&mut bytes)
        .map_err(|_| Error::Io(std::io::Error::other("OS randomness unavailable")))?;
    Ok(u16::from_ne_bytes(bytes))
}

```
