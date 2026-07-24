---
type: "code-fragment"
fragment_id: "rgbdns-frag-c86fa04276bf"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "validate_aliases"
kind: "fn"
start_line: 61
end_line: 84
---

# validate_aliases

- Fragment ID: `rgbdns-frag-c86fa04276bf`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 61-84
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c86fa04276bf", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-c86fa04276bf: fn validate_aliases", "sourcePath": "src/zone.rs", "startLine": 61, "endLine": 84}
```

## Excerpt

<span id="rgbdns-frag-c86fa04276bf" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c86fa04276bf: fn validate_aliases

```rust
    fn validate_aliases(&self) -> Result<()> {
        for (owner, records) in &self.records {
            let cnames = records
                .iter()
                .filter(|record| record.rr_type() == RecordType::Cname)
                .collect::<Vec<_>>();
            if cnames.is_empty() {
                continue;
            }
            if records
                .iter()
                .any(|record| record.rr_type() != RecordType::Cname)
                || cnames
                    .iter()
                    .skip(1)
                    .any(|record| record.data != cnames[0].data)
            {
                return Err(Error::InvalidRecord(format!(
                    "CNAME at {owner} conflicts with other data"
                )));
            }
        }
        Ok(())
    }
```
