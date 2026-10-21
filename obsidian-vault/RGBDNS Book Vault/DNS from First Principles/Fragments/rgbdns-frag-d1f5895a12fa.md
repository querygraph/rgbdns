---
type: "code-fragment"
fragment_id: "rgbdns-frag-d1f5895a12fa"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "lookup_from"
kind: "fn"
start_line: 685
end_line: 687
---

# lookup_from

- Fragment ID: `rgbdns-frag-d1f5895a12fa`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 685-687
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d1f5895a12fa", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 685, "endLine": 687}
```

## Excerpt

<span id="rgbdns-frag-d1f5895a12fa" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d1f5895a12fa: fn lookup_from

```rust
    pub fn lookup_from(&self, name: &Name, typ: RecordType, client: IpAddr) -> Lookup {
        self.lookup_for(name, typ, Some(client), unix_now())
    }
```
