---
type: "code-fragment"
fragment_id: "rgbdns-frag-ca6aa7f85b39"
source_path: "src/conf.rs"
code_note: "DNS from First Principles/Code/src/conf.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "executable"
kind: "fn"
start_line: 223
end_line: 230
---

# executable

- Fragment ID: `rgbdns-frag-ca6aa7f85b39`
- Source file: [[DNS from First Principles/Code/src/conf.rs.source|src/conf.rs]]
- Lines: 223-230
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ca6aa7f85b39", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-ca6aa7f85b39: fn executable", "sourcePath": "src/conf.rs", "startLine": 223, "endLine": 230}
```

## Excerpt

<span id="rgbdns-frag-ca6aa7f85b39" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ca6aa7f85b39: fn executable

```rust
fn executable(name: &str) -> Result<PathBuf> {
    let current = std::env::current_exe()?;
    Ok(current
        .parent()
        .ok_or(Error::Format("configuration executable has no parent"))?
        .join(name))
}

```
