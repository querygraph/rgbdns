---
type: "code-fragment"
fragment_id: "rgbdns-frag-f318af8bdeaa"
source_path: "src/cdb.rs"
code_note: "DNS from First Principles/Code/src/cdb.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "compile"
kind: "fn"
start_line: 12
end_line: 61
---

# compile

- Fragment ID: `rgbdns-frag-f318af8bdeaa`
- Source file: [[DNS from First Principles/Code/src/cdb.rs.source|src/cdb.rs]]
- Lines: 12-61
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f318af8bdeaa", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-f318af8bdeaa: fn compile", "sourcePath": "src/cdb.rs", "startLine": 12, "endLine": 61}
```

## Excerpt

<span id="rgbdns-frag-f318af8bdeaa" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f318af8bdeaa: fn compile

```rust
pub fn compile(zone: &Zone, path: impl AsRef<Path>) -> Result<()> {
    let filename = path.as_ref().to_string_lossy().into_owned();
    let mut writer = cdb::CDBWriter::create(filename)
        .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    for (prefix, location) in zone.location_entries() {
        let mut key = b"\0%".to_vec();
        key.extend(prefix);
        writer
            .add(&key, &location)
            .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    }
    for (owner, aname) in zone.aname_entries() {
        let mut key = b"\0A".to_vec();
        key.extend(owner.to_wire());
        let mut value = aname.ttl.to_be_bytes().to_vec();
        value.extend(aname.target.to_wire());
        writer
            .add(&key, &value)
            .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    }
    for (record, metadata) in zone.record_entries() {
        if record.rr_type() == RecordType::Opt {
            continue;
        }
        let (owner, marker) = match record.name.without_wildcard() {
            Some(parent) => (parent, b'*'),
            None => (record.name.clone(), b'='),
        };
        let mut value = Vec::new();
        value.extend(record.rr_type().code().to_be_bytes());
        value.push(if metadata.location.is_some() {
            marker + 1
        } else {
            marker
        });
        if let Some(location) = metadata.location {
            value.extend(location);
        }
        value.extend(record.ttl.to_be_bytes());
        value.extend(metadata.cutoff.to_be_bytes());
        encode_rdata(&record.data, &mut value)?;
        writer
            .add(&owner.to_wire(), &value)
            .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    }
    writer
        .finish()
        .map_err(|error| Error::Io(std::io::Error::other(error)))
}

```
