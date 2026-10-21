---
type: "code-fragment"
fragment_id: "rgbdns-frag-3e116c5d2267"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "snapshot"
kind: "fn"
start_line: 51
end_line: 54
---

# snapshot

- Fragment ID: `rgbdns-frag-3e116c5d2267`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 51-54
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3e116c5d2267", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 51, "endLine": 54}
```

## Excerpt

<span id="rgbdns-frag-3e116c5d2267" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3e116c5d2267: fn snapshot

```rust
    pub fn snapshot(&self) -> Arc<Zone> {
        self.0.read().expect("live zone lock poisoned").clone()
    }

```
