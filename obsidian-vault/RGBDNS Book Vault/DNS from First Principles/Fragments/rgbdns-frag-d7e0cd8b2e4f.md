---
type: "code-fragment"
fragment_id: "rgbdns-frag-d7e0cd8b2e4f"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "name_exists"
kind: "fn"
start_line: 601
end_line: 613
---

# name_exists

- Fragment ID: `rgbdns-frag-d7e0cd8b2e4f`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 601-613
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d7e0cd8b2e4f", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-d7e0cd8b2e4f: fn name_exists", "sourcePath": "src/zone.rs", "startLine": 601, "endLine": 613}
```

## Excerpt

<span id="rgbdns-frag-d7e0cd8b2e4f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d7e0cd8b2e4f: fn name_exists

```rust
    fn name_exists(&self, name: &Name, location: [u8; 2], now: u64) -> bool {
        if self.unqualified_nodes.contains(name) {
            return true;
        }
        if !self.nodes.contains(name) {
            return false;
        }
        self.records.keys().any(|owner| {
            owner.is_subdomain_of(name) && !self.visible_records(owner, location, now).is_empty()
        })
    }
}

```
