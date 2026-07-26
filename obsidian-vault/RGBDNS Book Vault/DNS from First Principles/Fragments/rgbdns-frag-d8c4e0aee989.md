---
type: "code-fragment"
fragment_id: "rgbdns-frag-d8c4e0aee989"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "lookup"
kind: "fn"
start_line: 529
end_line: 531
---

# lookup

- Fragment ID: `rgbdns-frag-d8c4e0aee989`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 529-531
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d8c4e0aee989", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-d8c4e0aee989: fn lookup", "sourcePath": "src/zone.rs", "startLine": 529, "endLine": 531}
```

## Excerpt

<span id="rgbdns-frag-d8c4e0aee989" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d8c4e0aee989: fn lookup

```rust
    pub fn lookup(&self, name: &Name, typ: RecordType) -> Lookup {
        self.lookup_for(name, typ, None, unix_now())
    }
```
