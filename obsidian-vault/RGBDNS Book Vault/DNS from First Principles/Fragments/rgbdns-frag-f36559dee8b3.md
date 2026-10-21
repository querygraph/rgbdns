---
type: "code-fragment"
fragment_id: "rgbdns-frag-f36559dee8b3"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "addresses"
kind: "fn"
start_line: 162
end_line: 214
---

# addresses

- Fragment ID: `rgbdns-frag-f36559dee8b3`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 162-214
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f36559dee8b3", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "sourcePath": "src/aname.rs", "startLine": 162, "endLine": 214}
```

## Excerpt

<span id="rgbdns-frag-f36559dee8b3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f36559dee8b3: fn addresses

```rust
fn addresses(
    answers: &[Record],
    target: &Name,
    record_type: RecordType,
) -> Result<(Vec<RData>, Option<u32>)> {
    let mut current = target.clone();
    let mut visited = HashSet::new();
    let mut ttl = None;
    for _ in 0..MAX_CHAIN {
        if !visited.insert(current.clone()) {
            return Err(Error::Format("ANAME target contains a CNAME loop"));
        }
        let values = answers
            .iter()
            .filter(|record| record.name == current && record.rr_type() == record_type)
            .take(MAX_ADDRESSES + 1)
            .collect::<Vec<_>>();
        if values.len() > MAX_ADDRESSES {
            return Err(Error::Format("ANAME target has too many addresses"));
        }
        if !values.is_empty() {
            for record in &values {
                ttl = Some(ttl.map_or(record.ttl, |value: u32| value.min(record.ttl)));
            }
            return Ok((
                values
                    .into_iter()
                    .map(|record| record.data.clone())
                    .collect(),
                ttl,
            ));
        }
        let aliases = answers
            .iter()
            .filter_map(|record| match &record.data {
                RData::Name(RecordType::Cname, next) if record.name == current => {
                    Some((next, record.ttl))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        if aliases.is_empty() {
            return Ok((Vec::new(), ttl));
        }
        if aliases.iter().any(|(next, _)| *next != aliases[0].0) {
            return Err(Error::Format("ANAME target has conflicting CNAME answers"));
        }
        ttl = Some(ttl.map_or(aliases[0].1, |value: u32| value.min(aliases[0].1)));
        current = aliases[0].0.clone();
    }
    Err(Error::Format("ANAME target CNAME chain is too long"))
}

```
