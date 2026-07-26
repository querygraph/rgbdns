---
type: "code-fragment"
fragment_id: "rgbdns-frag-4c56545e9677"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "from_file"
kind: "fn"
start_line: 36
end_line: 48
---

# from_file

- Fragment ID: `rgbdns-frag-4c56545e9677`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 36-48
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4c56545e9677", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-4c56545e9677: fn from_file", "sourcePath": "src/zone.rs", "startLine": 36, "endLine": 48}
```

## Excerpt

<span id="rgbdns-frag-4c56545e9677" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4c56545e9677: fn from_file

```rust
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if path.extension().is_some_and(|extension| extension == "cdb") {
            crate::cdb::load(path)
        } else {
            let serial = fs::metadata(path)?
                .modified()?
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .map_or(1, |duration| duration.as_secs() as u32)
                .max(1);
            Self::parse_with_serial(&fs::read_to_string(path)?, serial)
        }
    }
```
