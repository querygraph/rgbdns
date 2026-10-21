---
type: "code-fragment"
fragment_id: "rgbdns-frag-ddb2ef73889a"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "has_qualified_data_in_zone"
kind: "fn"
start_line: 263
end_line: 274
---

# has_qualified_data_in_zone

- Fragment ID: `rgbdns-frag-ddb2ef73889a`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 263-274
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ddb2ef73889a", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 263, "endLine": 274}
```

## Excerpt

<span id="rgbdns-frag-ddb2ef73889a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ddb2ef73889a: fn has_qualified_data_in_zone

```rust
    pub(crate) fn has_qualified_data_in_zone(&self, name: &Name) -> bool {
        self.records.iter().any(|(owner, records)| {
            owner.is_subdomain_of(name)
                && !self.authoritative.iter().any(|child| {
                    child != name && child.is_subdomain_of(name) && owner.is_subdomain_of(child)
                })
                && records.iter().enumerate().any(|(index, _)| {
                    let metadata = self.metadata[owner][index];
                    metadata.location.is_some() || metadata.cutoff != 0
                })
        })
    }
```
