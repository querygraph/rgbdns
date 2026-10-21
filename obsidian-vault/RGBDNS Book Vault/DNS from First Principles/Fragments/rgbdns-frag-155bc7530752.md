---
type: "code-fragment"
fragment_id: "rgbdns-frag-155bc7530752"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "dnssec_signatures"
kind: "fn"
start_line: 854
end_line: 890
---

# dnssec_signatures

- Fragment ID: `rgbdns-frag-155bc7530752`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 854-890
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-155bc7530752", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 854, "endLine": 890}
```

## Excerpt

<span id="rgbdns-frag-155bc7530752" class="rgbdns-fragment-target"></span>
### rgbdns-frag-155bc7530752: fn dnssec_signatures

```rust
    pub(crate) fn dnssec_signatures(
        &self,
        name: &Name,
        covered: &BTreeSet<RecordType>,
        client: Option<IpAddr>,
    ) -> Vec<Record> {
        let location = self.client_location(client);
        let now = 4_611_686_018_427_387_914u64.saturating_add(unix_now());
        let mut owner = name.clone();
        let mut signatures = self.visible_records(&owner, location, now);
        if !signatures
            .iter()
            .any(|record| record.rr_type() == RecordType::Rrsig)
        {
            let mut parent = name.parent();
            while let Some(candidate) = parent {
                if self.name_exists(&candidate, location, now) {
                    owner = candidate.wildcard();
                    signatures = self.visible_records(&owner, location, now);
                    break;
                }
                parent = candidate.parent();
            }
        }
        signatures
            .into_iter()
            .filter(|record| {
                record.rr_type() == RecordType::Rrsig
                    && rrsig_covered(&record.data).is_some_and(|typ| covered.contains(&typ))
            })
            .map(|mut record| {
                record.name = name.clone();
                record
            })
            .collect()
    }

```
