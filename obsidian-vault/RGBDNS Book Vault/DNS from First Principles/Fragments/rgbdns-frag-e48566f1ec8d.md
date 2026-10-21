---
type: "code-fragment"
fragment_id: "rgbdns-frag-e48566f1ec8d"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "add_section_signatures"
kind: "fn"
start_line: 378
end_line: 395
---

# add_section_signatures

- Fragment ID: `rgbdns-frag-e48566f1ec8d`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 378-395
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e48566f1ec8d", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 378, "endLine": 395}
```

## Excerpt

<span id="rgbdns-frag-e48566f1ec8d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e48566f1ec8d: fn add_section_signatures

```rust
fn add_section_signatures(zone: &Zone, section: &mut Vec<crate::Record>, client: Option<IpAddr>) {
    let rrsets = section
        .iter()
        .filter(|record| record.rr_type() != crate::RecordType::Rrsig)
        .fold(
            HashMap::<crate::Name, std::collections::BTreeSet<_>>::new(),
            |mut sets, record| {
                sets.entry(record.name.clone())
                    .or_default()
                    .insert(record.rr_type());
                sets
            },
        );
    for (name, covered) in rrsets {
        section.extend(zone.dnssec_signatures(&name, &covered, client));
    }
}

```
