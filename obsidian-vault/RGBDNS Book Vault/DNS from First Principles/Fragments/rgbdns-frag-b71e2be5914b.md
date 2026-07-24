---
type: "code-fragment"
fragment_id: "rgbdns-frag-b71e2be5914b"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "record_entries"
kind: "fn"
start_line: 100
end_line: 110
---

# record_entries

- Fragment ID: `rgbdns-frag-b71e2be5914b`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 100-110
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b71e2be5914b", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-b71e2be5914b: fn record_entries", "sourcePath": "src/zone.rs", "startLine": 100, "endLine": 110}
```

## Excerpt

<span id="rgbdns-frag-b71e2be5914b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b71e2be5914b: fn record_entries

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
