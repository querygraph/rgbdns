---
type: "code-fragment"
fragment_id: "rgbdns-frag-0495392f2c71"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "record_entries"
kind: "fn"
start_line: 215
end_line: 225
---

# record_entries

- Fragment ID: `rgbdns-frag-0495392f2c71`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 215-225
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0495392f2c71", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 215, "endLine": 225}
```

## Excerpt

<span id="rgbdns-frag-0495392f2c71" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0495392f2c71: fn record_entries

```rust
    pub(crate) fn record_entries(&self) -> impl Iterator<Item = (&Record, RecordMetadata)> {
        self.records.iter().flat_map(|(owner, records)| {
            records.iter().zip(
                self.metadata
                    .get(owner)
                    .expect("record metadata invariant")
                    .iter()
                    .copied(),
            )
        })
    }
```
