---
type: "code-fragment"
fragment_id: "rgbdns-frag-d16edb4202b0"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "authoritative_names"
kind: "fn"
start_line: 275
end_line: 277
---

# authoritative_names

- Fragment ID: `rgbdns-frag-d16edb4202b0`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 275-277
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d16edb4202b0", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 275, "endLine": 277}
```

## Excerpt

<span id="rgbdns-frag-d16edb4202b0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d16edb4202b0: fn authoritative_names

```rust
    pub(crate) fn authoritative_names(&self) -> BTreeSet<Name> {
        self.authoritative.clone()
    }
```
