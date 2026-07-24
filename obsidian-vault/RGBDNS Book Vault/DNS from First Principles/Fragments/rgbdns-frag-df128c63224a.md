---
type: "code-fragment"
fragment_id: "rgbdns-frag-df128c63224a"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "lookup"
kind: "fn"
start_line: 459
end_line: 461
---

# lookup

- Fragment ID: `rgbdns-frag-df128c63224a`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 459-461
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-df128c63224a", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-df128c63224a: fn lookup", "sourcePath": "src/zone.rs", "startLine": 459, "endLine": 461}
```

## Excerpt

<span id="rgbdns-frag-df128c63224a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-df128c63224a: fn lookup

```rust
    pub fn lookup(&self, name: &Name, typ: RecordType) -> Lookup {
        self.lookup_for(name, typ, None, unix_now())
    }
```
