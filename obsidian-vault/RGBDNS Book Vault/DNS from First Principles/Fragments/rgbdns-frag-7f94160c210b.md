---
type: "code-fragment"
fragment_id: "rgbdns-frag-7f94160c210b"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "materialization_input"
kind: "fn"
start_line: 288
end_line: 306
---

# materialization_input

- Fragment ID: `rgbdns-frag-7f94160c210b`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 288-306
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7f94160c210b", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 288, "endLine": 306}
```

## Excerpt

<span id="rgbdns-frag-7f94160c210b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7f94160c210b: fn materialization_input

```rust
    pub(crate) fn materialization_input(&self) -> Result<MaterializationInput> {
        if self
            .record_entries()
            .any(|(_, metadata)| metadata.location.is_some() || metadata.cutoff != 0)
        {
            return Err(Error::InvalidRecord(
                "ANAME materialization does not accept location-dependent or expiring data".into(),
            ));
        }
        Ok((
            self.record_entries()
                .map(|(record, _)| record.clone())
                .collect(),
            self.anames
                .iter()
                .map(|(owner, aname)| (owner.clone(), aname.clone()))
                .collect(),
        ))
    }
```
