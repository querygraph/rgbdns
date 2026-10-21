---
type: "code-fragment"
fragment_id: "rgbdns-frag-cd7a8aca66f3"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "records_outside"
kind: "fn"
start_line: 278
end_line: 287
---

# records_outside

- Fragment ID: `rgbdns-frag-cd7a8aca66f3`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 278-287
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-cd7a8aca66f3", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 278, "endLine": 287}
```

## Excerpt

<span id="rgbdns-frag-cd7a8aca66f3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-cd7a8aca66f3: fn records_outside

```rust
    pub(crate) fn records_outside(&self, zones: &BTreeSet<Name>) -> Vec<Record> {
        self.record_entries()
            .filter(|(record, metadata)| {
                metadata.location.is_none()
                    && metadata.cutoff == 0
                    && !zones.iter().any(|zone| record.name.is_subdomain_of(zone))
            })
            .map(|(record, _)| record.clone())
            .collect()
    }
```
