---
type: "code-fragment"
fragment_id: "rgbdns-frag-30f7b9055a86"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "list_overlay"
kind: "fn"
start_line: 183
end_line: 189
---

# list_overlay

- Fragment ID: `rgbdns-frag-30f7b9055a86`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 183-189
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-30f7b9055a86", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 183, "endLine": 189}
```

## Excerpt

<span id="rgbdns-frag-30f7b9055a86" class="rgbdns-fragment-target"></span>
### rgbdns-frag-30f7b9055a86: fn list_overlay

```rust
pub fn list_overlay(state_dir: impl AsRef<Path>) -> Result<Vec<(Name, Vec<u8>)>> {
    Ok(read_overlay(&state_dir.as_ref().join("acme-overlay.data"))?
        .into_iter()
        .flat_map(|(owner, values)| values.into_iter().map(move |value| (owner.clone(), value)))
        .collect())
}

```
