---
type: "code-fragment"
fragment_id: "rgbdns-frag-83b463908a3c"
source_path: "src/cdb.rs"
code_note: "DNS from First Principles/Code/src/cdb.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "load"
kind: "fn"
start_line: 53
end_line: 69
---

# load

- Fragment ID: `rgbdns-frag-83b463908a3c`
- Source file: [[DNS from First Principles/Code/src/cdb.rs.source|src/cdb.rs]]
- Lines: 53-69
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-83b463908a3c", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-83b463908a3c: fn load", "sourcePath": "src/cdb.rs", "startLine": 53, "endLine": 69}
```

## Excerpt

<span id="rgbdns-frag-83b463908a3c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-83b463908a3c: fn load

```rust
pub fn load(path: impl AsRef<Path>) -> Result<Zone> {
    let entries = read_entries(path)?;
    let mut records = Vec::new();
    let mut locations = Vec::new();
    for (key, value) in entries {
        if key.starts_with(b"\0%") {
            if value.len() != 2 || key.len() > 6 {
                return Err(Error::Format("invalid location mapping"));
            }
            locations.push((key[2..].to_vec(), [value[0], value[1]]));
            continue;
        }
        records.push(decode_record(&key, &value)?);
    }
    Ok(Zone::from_compiled_records(records, locations))
}

```
