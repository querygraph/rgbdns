---
type: "code-fragment"
fragment_id: "rgbdns-frag-eecbf1bac201"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "State"
kind: "struct"
start_line: 213
end_line: 219
---

# State

- Fragment ID: `rgbdns-frag-eecbf1bac201`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 213-219
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-eecbf1bac201", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 213, "endLine": 219}
```

## Excerpt

<span id="rgbdns-frag-eecbf1bac201" class="rgbdns-fragment-target"></span>
### rgbdns-frag-eecbf1bac201: struct State

```rust
struct State {
    overlay: Overlay,
    serials: BTreeMap<Name, u32>,
    recent_macs: VecDeque<(u64, Vec<u8>)>,
}

#[derive(Clone, Debug)]
```
