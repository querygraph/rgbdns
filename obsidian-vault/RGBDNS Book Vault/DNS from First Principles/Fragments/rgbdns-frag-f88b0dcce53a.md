---
type: "code-fragment"
fragment_id: "rgbdns-frag-f88b0dcce53a"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "visible_records"
kind: "fn"
start_line: 627
end_line: 652
---

# visible_records

- Fragment ID: `rgbdns-frag-f88b0dcce53a`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 627-652
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f88b0dcce53a", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-f88b0dcce53a: fn visible_records", "sourcePath": "src/zone.rs", "startLine": 627, "endLine": 652}
```

## Excerpt

<span id="rgbdns-frag-f88b0dcce53a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f88b0dcce53a: fn visible_records

```rust
    fn visible_records(&self, owner: &Name, location: [u8; 2], now: u64) -> Vec<Record> {
        let Some(records) = self.records.get(owner) else {
            return Vec::new();
        };
        records
            .iter()
            .zip(self.metadata.get(owner).expect("record metadata invariant"))
            .filter_map(|(record, metadata)| {
                if metadata.location.is_some_and(|value| value != location) {
                    return None;
                }
                let mut record = record.clone();
                if metadata.cutoff != 0 {
                    if record.ttl == 0 {
                        if metadata.cutoff < now {
                            return None;
                        }
                        record.ttl = metadata.cutoff.saturating_sub(now).clamp(2, 3600) as u32;
                    } else if metadata.cutoff >= now {
                        return None;
                    }
                }
                Some(record)
            })
            .collect()
    }
```
