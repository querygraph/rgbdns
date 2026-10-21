---
type: "code-fragment"
fragment_id: "rgbdns-frag-3b922ac55a4a"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "read_serials"
kind: "fn"
start_line: 733
end_line: 753
---

# read_serials

- Fragment ID: `rgbdns-frag-3b922ac55a4a`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 733-753
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3b922ac55a4a", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 733, "endLine": 753}
```

## Excerpt

<span id="rgbdns-frag-3b922ac55a4a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3b922ac55a4a: fn read_serials

```rust
fn read_serials(path: &Path) -> Result<BTreeMap<Name, u32>> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(BTreeMap::new()),
        Err(error) => return Err(error.into()),
    };
    text.lines()
        .map(|line| {
            let (name, serial) = line
                .split_once('\t')
                .ok_or(Error::Format("invalid ACME serial state"))?;
            Ok((
                Name::from_str(name)?,
                serial
                    .parse::<u32>()
                    .map_err(|_| Error::Format("invalid ACME serial"))?,
            ))
        })
        .collect()
}

```
