---
type: "code-fragment"
fragment_id: "rgbdns-frag-ed0bf3adfed0"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "name_exists"
kind: "fn"
start_line: 671
end_line: 683
---

# name_exists

- Fragment ID: `rgbdns-frag-ed0bf3adfed0`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 671-683
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ed0bf3adfed0", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-ed0bf3adfed0: fn name_exists", "sourcePath": "src/zone.rs", "startLine": 671, "endLine": 683}
```

## Excerpt

<span id="rgbdns-frag-ed0bf3adfed0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ed0bf3adfed0: fn name_exists

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
