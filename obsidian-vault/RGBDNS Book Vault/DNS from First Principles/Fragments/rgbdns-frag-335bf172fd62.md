---
type: "code-fragment"
fragment_id: "rgbdns-frag-335bf172fd62"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "parse_with_serial"
kind: "fn"
start_line: 45
end_line: 60
---

# parse_with_serial

- Fragment ID: `rgbdns-frag-335bf172fd62`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 45-60
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-335bf172fd62", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-335bf172fd62: fn parse_with_serial", "sourcePath": "src/zone.rs", "startLine": 45, "endLine": 60}
```

## Excerpt

<span id="rgbdns-frag-335bf172fd62" class="rgbdns-fragment-target"></span>
### rgbdns-frag-335bf172fd62: fn parse_with_serial

```rust
    fn parse_with_serial(text: &str, default_serial: u32) -> Result<Self> {
        let mut z = Self {
            default_serial,
            ..Self::default()
        };
        for (number, raw) in text.lines().enumerate() {
            let line = raw.trim_end_matches('\r');
            if line.is_empty() || line.starts_with('#') || line.starts_with('-') {
                continue;
            }
            z.add_line(line)
                .map_err(|e| Error::InvalidRecord(format!("line {}: {e}", number + 1)))?;
        }
        z.validate_aliases()?;
        Ok(z)
    }
```
