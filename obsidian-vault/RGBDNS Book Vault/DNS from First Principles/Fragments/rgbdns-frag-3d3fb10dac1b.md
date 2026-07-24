---
type: "code-fragment"
fragment_id: "rgbdns-frag-3d3fb10dac1b"
source_path: "src/client.rs"
code_note: "DNS from First Principles/Code/src/client.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "random_id"
kind: "fn"
start_line: 140
end_line: 147
---

# random_id

- Fragment ID: `rgbdns-frag-3d3fb10dac1b`
- Source file: [[DNS from First Principles/Code/src/client.rs.source|src/client.rs]]
- Lines: 140-147
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3d3fb10dac1b", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "heading": "rgbdns-frag-3d3fb10dac1b: fn random_id", "sourcePath": "src/client.rs", "startLine": 140, "endLine": 147}
```

## Excerpt

<span id="rgbdns-frag-3d3fb10dac1b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3d3fb10dac1b: fn random_id

```rust
fn random_id() -> Result<u16> {
    let mut bytes = [0; 2];
    getrandom::fill(&mut bytes)
        .map_err(|_| Error::Io(std::io::Error::other("OS randomness unavailable")))?;
    Ok(u16::from_ne_bytes(bytes))
}

#[cfg(test)]
```
