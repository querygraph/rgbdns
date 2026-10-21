---
type: "code-fragment"
fragment_id: "rgbdns-frag-55f1813f20d1"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "publish"
kind: "fn"
start_line: 55
end_line: 59
---

# publish

- Fragment ID: `rgbdns-frag-55f1813f20d1`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 55-59
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-55f1813f20d1", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 55, "endLine": 59}
```

## Excerpt

<span id="rgbdns-frag-55f1813f20d1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-55f1813f20d1: fn publish

```rust
    fn publish(&self, zone: Zone) {
        *self.0.write().expect("live zone lock poisoned") = Arc::new(zone);
    }
}

```
