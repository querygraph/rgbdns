---
type: "code-fragment"
fragment_id: "rgbdns-frag-b06ef1bbad8b"
source_path: "src/pick.rs"
code_note: "DNS from First Principles/Code/src/pick.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "compile"
kind: "fn"
start_line: 157
end_line: 184
---

# compile

- Fragment ID: `rgbdns-frag-b06ef1bbad8b`
- Source file: [[DNS from First Principles/Code/src/pick.rs.source|src/pick.rs]]
- Lines: 157-184
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b06ef1bbad8b", "codeNote": "DNS from First Principles/Code/src/pick.rs.source", "sourcePath": "src/pick.rs", "startLine": 157, "endLine": 184}
```

## Excerpt

<span id="rgbdns-frag-b06ef1bbad8b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b06ef1bbad8b: fn compile

```rust
pub fn compile(database: &Database, path: impl AsRef<Path>) -> Result<()> {
    let filename = path.as_ref().to_string_lossy().into_owned();
    let mut writer = cdb::CDBWriter::create(filename)
        .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    for (prefix, location) in &database.locations {
        let mut key = vec![b'%'];
        key.extend(prefix);
        writer
            .add(&key, location)
            .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    }
    for ((location, name), addresses) in &database.addresses {
        let mut key = vec![b'+'];
        key.extend(location);
        key.extend(name.to_wire());
        let value = addresses
            .iter()
            .flat_map(|address| address.octets())
            .collect::<Vec<_>>();
        writer
            .add(&key, &value)
            .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    }
    writer
        .finish()
        .map_err(|error| Error::Io(std::io::Error::other(error)))
}

```
