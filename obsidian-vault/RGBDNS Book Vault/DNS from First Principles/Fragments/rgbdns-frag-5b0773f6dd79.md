---
type: "code-fragment"
fragment_id: "rgbdns-frag-5b0773f6dd79"
source_path: "src/rbl.rs"
code_note: "DNS from First Principles/Code/src/rbl.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "listed"
kind: "fn"
start_line: 82
end_line: 93
---

# listed

- Fragment ID: `rgbdns-frag-5b0773f6dd79`
- Source file: [[DNS from First Principles/Code/src/rbl.rs.source|src/rbl.rs]]
- Lines: 82-93
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5b0773f6dd79", "codeNote": "DNS from First Principles/Code/src/rbl.rs.source", "heading": "rgbdns-frag-5b0773f6dd79: fn listed", "sourcePath": "src/rbl.rs", "startLine": 82, "endLine": 93}
```

## Excerpt

<span id="rgbdns-frag-5b0773f6dd79" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5b0773f6dd79: fn listed

```rust
    fn listed(&self, address: Ipv4Addr) -> bool {
        let address = u32::from(address);
        (8..=32).rev().any(|prefix| {
            let mask = if prefix == 0 {
                0
            } else {
                u32::MAX << (32 - prefix)
            };
            self.networks.contains(&(address & mask, prefix))
        })
    }

```
