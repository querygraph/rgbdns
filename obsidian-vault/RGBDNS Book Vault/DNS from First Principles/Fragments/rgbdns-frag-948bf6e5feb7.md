---
type: "code-fragment"
fragment_id: "rgbdns-frag-948bf6e5feb7"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "lookup_from"
kind: "fn"
start_line: 532
end_line: 534
---

# lookup_from

- Fragment ID: `rgbdns-frag-948bf6e5feb7`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 532-534
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-948bf6e5feb7", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-948bf6e5feb7: fn lookup_from", "sourcePath": "src/zone.rs", "startLine": 532, "endLine": 534}
```

## Excerpt

<span id="rgbdns-frag-948bf6e5feb7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-948bf6e5feb7: fn lookup_from

```rust
    pub fn lookup_from(&self, name: &Name, typ: RecordType, client: IpAddr) -> Lookup {
        self.lookup_for(name, typ, Some(client), unix_now())
    }
```
