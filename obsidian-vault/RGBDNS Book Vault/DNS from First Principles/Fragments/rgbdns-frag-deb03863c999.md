---
type: "code-fragment"
fragment_id: "rgbdns-frag-deb03863c999"
source_path: "src/pick.rs"
code_note: "DNS from First Principles/Code/src/pick.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "compile"
kind: "fn"
start_line: 155
end_line: 182
---

# compile

- Fragment ID: `rgbdns-frag-deb03863c999`
- Source file: [[DNS from First Principles/Code/src/pick.rs.source|src/pick.rs]]
- Lines: 155-182
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-deb03863c999", "codeNote": "DNS from First Principles/Code/src/pick.rs.source", "heading": "rgbdns-frag-deb03863c999: fn compile", "sourcePath": "src/pick.rs", "startLine": 155, "endLine": 182}
```

## Excerpt

<span id="rgbdns-frag-deb03863c999" class="rgbdns-fragment-target"></span>
### rgbdns-frag-deb03863c999: fn compile

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
