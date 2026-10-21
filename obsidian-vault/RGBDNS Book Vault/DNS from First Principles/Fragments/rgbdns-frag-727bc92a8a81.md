---
type: "code-fragment"
fragment_id: "rgbdns-frag-727bc92a8a81"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "covered"
kind: "fn"
start_line: 136
end_line: 143
---

# covered

- Fragment ID: `rgbdns-frag-727bc92a8a81`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 136-143
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-727bc92a8a81", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 136, "endLine": 143}
```

## Excerpt

<span id="rgbdns-frag-727bc92a8a81" class="rgbdns-fragment-target"></span>
### rgbdns-frag-727bc92a8a81: fn covered

```rust
    fn covered(&self) -> BTreeSet<Name> {
        self.signing
            .iter()
            .map(|policy| policy.zone.clone())
            .chain(self.unsigned.iter().cloned())
            .collect()
    }

```
