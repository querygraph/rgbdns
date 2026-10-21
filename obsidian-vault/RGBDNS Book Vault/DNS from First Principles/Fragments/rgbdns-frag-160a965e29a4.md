---
type: "code-fragment"
fragment_id: "rgbdns-frag-160a965e29a4"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "PolicySet"
kind: "struct"
start_line: 130
end_line: 134
---

# PolicySet

- Fragment ID: `rgbdns-frag-160a965e29a4`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 130-134
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-160a965e29a4", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 130, "endLine": 134}
```

## Excerpt

<span id="rgbdns-frag-160a965e29a4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-160a965e29a4: struct PolicySet

```rust
pub struct PolicySet {
    pub signing: Vec<Policy>,
    pub unsigned: BTreeSet<Name>,
}

```
