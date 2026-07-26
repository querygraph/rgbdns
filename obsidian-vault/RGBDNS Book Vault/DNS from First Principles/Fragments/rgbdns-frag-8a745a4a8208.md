---
type: "code-fragment"
fragment_id: "rgbdns-frag-8a745a4a8208"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "zone_lookup"
kind: "fn"
start_line: 239
end_line: 250
---

# zone_lookup

- Fragment ID: `rgbdns-frag-8a745a4a8208`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 239-250
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-8a745a4a8208", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-8a745a4a8208: fn zone_lookup", "sourcePath": "src/server.rs", "startLine": 239, "endLine": 250}
```

## Excerpt

<span id="rgbdns-frag-8a745a4a8208" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8a745a4a8208: fn zone_lookup

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
