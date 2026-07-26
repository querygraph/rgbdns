---
type: "code-fragment"
fragment_id: "rgbdns-frag-4625cc9215e0"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "expand_cname_chain"
kind: "fn"
start_line: 251
end_line: 291
---

# expand_cname_chain

- Fragment ID: `rgbdns-frag-4625cc9215e0`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 251-291
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4625cc9215e0", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-4625cc9215e0: fn expand_cname_chain", "sourcePath": "src/server.rs", "startLine": 251, "endLine": 291}
```

## Excerpt

<span id="rgbdns-frag-4625cc9215e0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4625cc9215e0: fn expand_cname_chain

```rust
fn expand_cname_chain(
    zone: &Zone,
    response: &mut Message,
    record_type: crate::RecordType,
    client: Option<IpAddr>,
) -> bool {
    let mut visited = response
        .answers
        .iter()
        .map(|record| record.name.clone())
        .collect::<HashSet<_>>();
    for _ in 0..16 {
        if response
            .answers
            .iter()
            .any(|record| record.rr_type() == record_type)
        {
            return true;
        }
        let Some(target) = response.answers.iter().rev().find_map(|record| {
            if let crate::RData::Name(crate::RecordType::Cname, target) = &record.data {
                Some(target.clone())
            } else {
                None
            }
        }) else {
            return true;
        };
        if !visited.insert(target.clone()) {
            return false;
        }
        match zone_lookup(zone, &target, record_type, client) {
            Lookup::Answer(records) => response.answers.extend(records),
            Lookup::NoData(_) | Lookup::NxDomain(_) | Lookup::Referral { .. } | Lookup::Refused => {
                return true;
            }
        }
    }
    false
}

```
