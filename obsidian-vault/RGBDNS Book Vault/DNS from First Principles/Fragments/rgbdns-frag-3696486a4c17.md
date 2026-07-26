---
type: "code-fragment"
fragment_id: "rgbdns-frag-3696486a4c17"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "record_entries"
kind: "fn"
start_line: 131
end_line: 141
---

# record_entries

- Fragment ID: `rgbdns-frag-3696486a4c17`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 131-141
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3696486a4c17", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-3696486a4c17: fn record_entries", "sourcePath": "src/zone.rs", "startLine": 131, "endLine": 141}
```

## Excerpt

<span id="rgbdns-frag-3696486a4c17" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3696486a4c17: fn record_entries

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
