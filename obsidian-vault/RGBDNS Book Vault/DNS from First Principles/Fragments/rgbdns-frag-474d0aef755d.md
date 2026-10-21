---
type: "code-fragment"
fragment_id: "rgbdns-frag-474d0aef755d"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "zone_lookup"
kind: "fn"
start_line: 439
end_line: 450
---

# zone_lookup

- Fragment ID: `rgbdns-frag-474d0aef755d`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 439-450
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-474d0aef755d", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 439, "endLine": 450}
```

## Excerpt

<span id="rgbdns-frag-474d0aef755d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-474d0aef755d: fn zone_lookup

```rust
fn zone_lookup(
    zone: &Zone,
    name: &crate::Name,
    record_type: crate::RecordType,
    client: Option<IpAddr>,
) -> Lookup {
    client.map_or_else(
        || zone.lookup(name, record_type),
        |address| zone.lookup_from(name, record_type, address),
    )
}

```
