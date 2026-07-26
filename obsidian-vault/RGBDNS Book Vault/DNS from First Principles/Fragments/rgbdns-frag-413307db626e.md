---
type: "code-fragment"
fragment_id: "rgbdns-frag-413307db626e"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "normalize_rrsets"
kind: "fn"
start_line: 196
end_line: 219
---

# normalize_rrsets

- Fragment ID: `rgbdns-frag-413307db626e`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 196-219
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-413307db626e", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-413307db626e: fn normalize_rrsets", "sourcePath": "src/server.rs", "startLine": 196, "endLine": 219}
```

## Excerpt

<span id="rgbdns-frag-413307db626e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-413307db626e: fn normalize_rrsets

```rust
fn normalize_rrsets(records: &mut Vec<crate::Record>) {
    let mut ttls = HashMap::new();
    for record in records.iter() {
        ttls.entry((record.name.clone(), record.rr_type()))
            .and_modify(|ttl: &mut u32| *ttl = (*ttl).min(record.ttl))
            .or_insert(record.ttl);
    }
    for record in records.iter_mut() {
        record.ttl = ttls[&(record.name.clone(), record.rr_type())];
    }
    let mut index = 0;
    while index < records.len() {
        if records[..index].iter().any(|record| {
            record.name == records[index].name
                && record.rr_type() == records[index].rr_type()
                && record.data == records[index].data
        }) {
            records.remove(index);
        } else {
            index += 1;
        }
    }
}

```
