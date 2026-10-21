---
type: "code-fragment"
fragment_id: "rgbdns-frag-230fef373ce0"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "add"
kind: "fn"
start_line: 200
end_line: 214
---

# add

- Fragment ID: `rgbdns-frag-230fef373ce0`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 200-214
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-230fef373ce0", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 200, "endLine": 214}
```

## Excerpt

<span id="rgbdns-frag-230fef373ce0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-230fef373ce0: fn add

```rust
    fn add(&mut self, r: Record) {
        let mut node = Some(r.name.clone());
        while let Some(name) = node {
            self.nodes.insert(name.clone());
            if self.current_metadata.cutoff == 0 && self.current_metadata.location.is_none() {
                self.unqualified_nodes.insert(name.clone());
            }
            node = name.parent();
        }
        self.metadata
            .entry(r.name.clone())
            .or_default()
            .push(self.current_metadata);
        self.records.entry(r.name.clone()).or_default().push(r)
    }
```
