---
type: "code-fragment"
fragment_id: "rgbdns-frag-f4af87c101c0"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "validate_aliases"
kind: "fn"
start_line: 68
end_line: 115
---

# validate_aliases

- Fragment ID: `rgbdns-frag-f4af87c101c0`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 68-115
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f4af87c101c0", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-f4af87c101c0: fn validate_aliases", "sourcePath": "src/zone.rs", "startLine": 68, "endLine": 115}
```

## Excerpt

<span id="rgbdns-frag-f4af87c101c0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f4af87c101c0: fn validate_aliases

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
        for (owner, aname) in &self.anames {
            if owner.without_wildcard().is_some() {
                return Err(Error::InvalidRecord(format!(
                    "ANAME owner cannot be a wildcard: {owner}"
                )));
            }
            if owner == &aname.target {
                return Err(Error::InvalidRecord(format!(
                    "ANAME at {owner} points to itself"
                )));
            }
            if self.records.get(owner).is_some_and(|records| {
                records.iter().any(|record| {
                    matches!(
                        record.rr_type(),
                        RecordType::A | RecordType::Aaaa | RecordType::Cname
                    )
                })
            }) {
                return Err(Error::InvalidRecord(format!(
                    "ANAME at {owner} conflicts with A, AAAA, or CNAME data"
                )));
            }
        }
        Ok(())
    }
```
