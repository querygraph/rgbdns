---
type: "code-fragment"
fragment_id: "rgbdns-frag-4dd75158d5d2"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "add_target_addresses"
kind: "fn"
start_line: 251
end_line: 277
---

# add_target_addresses

- Fragment ID: `rgbdns-frag-4dd75158d5d2`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 251-277
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4dd75158d5d2", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-4dd75158d5d2: fn add_target_addresses", "sourcePath": "src/server.rs", "startLine": 251, "endLine": 277}
```

## Excerpt

<span id="rgbdns-frag-4dd75158d5d2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4dd75158d5d2: fn add_target_addresses

```rust
fn add_target_addresses(zone: &Zone, response: &mut Message, client: Option<IpAddr>) {
    let targets = response
        .answers
        .iter()
        .filter_map(|record| match &record.data {
            crate::RData::Name(crate::RecordType::Ns, target)
            | crate::RData::Mx(_, target)
            | crate::RData::Srv { target, .. } => Some(target.clone()),
            _ => None,
        })
        .collect::<HashSet<_>>();
    for target in targets {
        for record_type in [crate::RecordType::A, crate::RecordType::Aaaa] {
            if let Lookup::Answer(records) = zone_lookup(zone, &target, record_type, client) {
                response
                    .additionals
                    .extend(records.into_iter().filter(|record| {
                        matches!(
                            record.rr_type(),
                            crate::RecordType::A | crate::RecordType::Aaaa
                        )
                    }));
            }
        }
    }
}

```
