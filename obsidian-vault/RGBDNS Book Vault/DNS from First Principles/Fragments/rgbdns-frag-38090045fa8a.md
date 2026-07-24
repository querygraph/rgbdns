---
type: "code-fragment"
fragment_id: "rgbdns-frag-38090045fa8a"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "lookup_from"
kind: "fn"
start_line: 462
end_line: 464
---

# lookup_from

- Fragment ID: `rgbdns-frag-38090045fa8a`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 462-464
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-38090045fa8a", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-38090045fa8a: fn lookup_from", "sourcePath": "src/zone.rs", "startLine": 462, "endLine": 464}
```

## Excerpt

<span id="rgbdns-frag-38090045fa8a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-38090045fa8a: fn lookup_from

```rust
    pub fn lookup_from(&self, name: &Name, typ: RecordType, client: IpAddr) -> Lookup {
        self.lookup_for(name, typ, Some(client), unix_now())
    }
```
