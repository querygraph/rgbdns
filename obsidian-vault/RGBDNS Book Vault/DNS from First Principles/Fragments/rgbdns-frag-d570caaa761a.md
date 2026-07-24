---
type: "code-fragment"
fragment_id: "rgbdns-frag-d570caaa761a"
source_path: "src/rbl.rs"
code_note: "DNS from First Principles/Code/src/rbl.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "compile"
kind: "fn"
start_line: 152
end_line: 176
---

# compile

- Fragment ID: `rgbdns-frag-d570caaa761a`
- Source file: [[DNS from First Principles/Code/src/rbl.rs.source|src/rbl.rs]]
- Lines: 152-176
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d570caaa761a", "codeNote": "DNS from First Principles/Code/src/rbl.rs.source", "heading": "rgbdns-frag-d570caaa761a: fn compile", "sourcePath": "src/rbl.rs", "startLine": 152, "endLine": 176}
```

## Excerpt

<span id="rgbdns-frag-d570caaa761a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d570caaa761a: fn compile

```rust
pub fn compile(database: &Database, path: impl AsRef<Path>) -> Result<()> {
    let filename = path.as_ref().to_string_lossy().into_owned();
    let mut writer = cdb::CDBWriter::create(filename)
        .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    for (answer, text) in &database.responses {
        let mut value = answer.octets().to_vec();
        value.extend(text);
        writer
            .add(b"", &value)
            .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    }
    let mut networks = database.networks.iter().copied().collect::<Vec<_>>();
    networks.sort_unstable();
    for (address, prefix) in networks {
        let mut key = address.to_be_bytes().to_vec();
        key.push(prefix);
        writer
            .add(&key, b"")
            .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    }
    writer
        .finish()
        .map_err(|error| Error::Io(std::io::Error::other(error)))
}

```
