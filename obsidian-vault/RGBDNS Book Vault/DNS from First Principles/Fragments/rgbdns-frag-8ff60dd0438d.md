---
type: "code-fragment"
fragment_id: "rgbdns-frag-8ff60dd0438d"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "add_dnssec_records"
kind: "fn"
start_line: 339
end_line: 377
---

# add_dnssec_records

- Fragment ID: `rgbdns-frag-8ff60dd0438d`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 339-377
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-8ff60dd0438d", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 339, "endLine": 377}
```

## Excerpt

<span id="rgbdns-frag-8ff60dd0438d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8ff60dd0438d: fn add_dnssec_records

```rust
fn add_dnssec_records(
    zone: &Zone,
    question: &crate::Question,
    response: &mut Message,
    client: Option<IpAddr>,
) {
    add_section_signatures(zone, &mut response.answers, client);
    add_section_signatures(zone, &mut response.authorities, client);

    match response.flags & 0x000f {
        3 => response
            .authorities
            .extend(zone.dnssec_nsec_zone(&question.name, client)),
        0 if response.answers.is_empty() && response.flags & 0x0400 != 0 => {
            let mut proof = zone.dnssec_nsec_at(&question.name, client);
            if proof.is_empty() {
                proof = zone.dnssec_nsec_zone(&question.name, client);
            }
            response.authorities.extend(proof);
        }
        _ => {}
    }

    let wildcard_answer = response.answers.iter().any(|record| {
        if let crate::RData::Opaque(crate::RecordType::Rrsig, bytes) = &record.data {
            bytes
                .get(3)
                .is_some_and(|labels| usize::from(*labels) < record.name.labels().count())
        } else {
            false
        }
    });
    if wildcard_answer {
        response
            .authorities
            .extend(zone.dnssec_nsec_zone(&question.name, client));
    }
}

```
