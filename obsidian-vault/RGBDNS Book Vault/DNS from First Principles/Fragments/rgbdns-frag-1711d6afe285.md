---
type: "code-fragment"
fragment_id: "rgbdns-frag-1711d6afe285"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "add"
kind: "fn"
start_line: 116
end_line: 130
---

# add

- Fragment ID: `rgbdns-frag-1711d6afe285`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 116-130
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1711d6afe285", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-1711d6afe285: fn add", "sourcePath": "src/zone.rs", "startLine": 116, "endLine": 130}
```

## Excerpt

<span id="rgbdns-frag-1711d6afe285" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1711d6afe285: fn add

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
