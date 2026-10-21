---
type: "code-fragment"
fragment_id: "rgbdns-frag-d145f8bfd76e"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "lookup"
kind: "fn"
start_line: 682
end_line: 684
---

# lookup

- Fragment ID: `rgbdns-frag-d145f8bfd76e`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 682-684
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d145f8bfd76e", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 682, "endLine": 684}
```

## Excerpt

<span id="rgbdns-frag-d145f8bfd76e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d145f8bfd76e: fn lookup

```rust
    pub fn lookup(&self, name: &Name, typ: RecordType) -> Lookup {
        self.lookup_for(name, typ, None, unix_now())
    }
```
