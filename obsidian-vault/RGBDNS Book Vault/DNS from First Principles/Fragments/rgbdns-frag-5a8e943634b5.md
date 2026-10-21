---
type: "code-fragment"
fragment_id: "rgbdns-frag-5a8e943634b5"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "new"
kind: "fn"
start_line: 47
end_line: 50
---

# new

- Fragment ID: `rgbdns-frag-5a8e943634b5`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 47-50
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5a8e943634b5", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 47, "endLine": 50}
```

## Excerpt

<span id="rgbdns-frag-5a8e943634b5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5a8e943634b5: fn new

```rust
    pub fn new(zone: Zone) -> Self {
        Self(Arc::new(RwLock::new(Arc::new(zone))))
    }

```
