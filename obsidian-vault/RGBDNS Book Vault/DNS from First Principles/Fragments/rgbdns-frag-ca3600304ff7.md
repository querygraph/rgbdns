---
type: "code-fragment"
fragment_id: "rgbdns-frag-ca3600304ff7"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "transfer_anames"
kind: "fn"
start_line: 234
end_line: 252
---

# transfer_anames

- Fragment ID: `rgbdns-frag-ca3600304ff7`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 234-252
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ca3600304ff7", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 234, "endLine": 252}
```

## Excerpt

<span id="rgbdns-frag-ca3600304ff7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ca3600304ff7: fn transfer_anames

```rust
    pub(crate) fn transfer_anames(&self, name: &Name) -> Option<Vec<(Name, Aname)>> {
        if !self.authoritative.contains(name) {
            return None;
        }
        Some(
            self.anames
                .iter()
                .filter(|(owner, _)| {
                    owner.is_subdomain_of(name)
                        && !self.authoritative.iter().any(|child| {
                            child != name
                                && child.is_subdomain_of(name)
                                && owner.is_subdomain_of(child)
                        })
                })
                .map(|(owner, aname)| (owner.clone(), aname.clone()))
                .collect(),
        )
    }
```
