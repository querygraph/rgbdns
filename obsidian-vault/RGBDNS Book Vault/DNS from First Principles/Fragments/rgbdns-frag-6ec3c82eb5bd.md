---
type: "code-fragment"
fragment_id: "rgbdns-frag-6ec3c82eb5bd"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "read_overlay"
kind: "fn"
start_line: 701
end_line: 732
---

# read_overlay

- Fragment ID: `rgbdns-frag-6ec3c82eb5bd`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 701-732
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-6ec3c82eb5bd", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 701, "endLine": 732}
```

## Excerpt

<span id="rgbdns-frag-6ec3c82eb5bd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6ec3c82eb5bd: fn read_overlay

```rust
fn read_overlay(path: &Path) -> Result<Overlay> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(BTreeMap::new()),
        Err(error) => return Err(error.into()),
    };
    if text.len() > MAX_STATE_BYTES {
        return Err(Error::Format("ACME overlay is too large"));
    }
    let mut overlay = Overlay::new();
    for line in text
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let body = line
            .strip_prefix('\'')
            .ok_or(Error::Format("invalid ACME overlay line"))?;
        let fields = split_tinydns(body)?;
        if fields.len() != 3 {
            return Err(Error::Format("invalid ACME overlay line"));
        }
        let owner = Name::from_str(&fields[0])?;
        let value = unescape_tinydns(&fields[1])?;
        if value.is_empty() || value.len() > 255 {
            return Err(Error::Format("invalid ACME overlay TXT"));
        }
        overlay.entry(owner).or_default().insert(value);
    }
    validate_overlay_counts(&overlay)?;
    Ok(overlay)
}

```
