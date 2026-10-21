---
type: "code-fragment"
fragment_id: "rgbdns-frag-053279bf2ac4"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "transfer"
kind: "fn"
start_line: 307
end_line: 330
---

# transfer

- Fragment ID: `rgbdns-frag-053279bf2ac4`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 307-330
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-053279bf2ac4", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 307, "endLine": 330}
```

## Excerpt

<span id="rgbdns-frag-053279bf2ac4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-053279bf2ac4: fn transfer

```rust
    pub fn transfer(&self, name: &Name) -> Option<Vec<Record>> {
        if !self.authoritative.contains(name) {
            return None;
        }
        let now = 4_611_686_018_427_387_914u64.saturating_add(unix_now());
        let soa = self.soa(name, [0, 0], now)?;
        let mut records = vec![soa.clone()];
        records.extend(
            self.records
                .iter()
                .filter(|(owner, _)| {
                    owner.is_subdomain_of(name)
                        && !self.authoritative.iter().any(|child| {
                            child != name
                                && child.is_subdomain_of(name)
                                && owner.is_subdomain_of(child)
                        })
                })
                .flat_map(|(owner, _)| self.visible_records(owner, [0, 0], now))
                .filter(|record| !(record.name == *name && record.rr_type() == RecordType::Soa)),
        );
        records.push(soa);
        Some(records)
    }
```
