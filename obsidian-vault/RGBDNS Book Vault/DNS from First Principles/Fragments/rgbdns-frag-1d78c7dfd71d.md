---
type: "code-fragment"
fragment_id: "rgbdns-frag-1d78c7dfd71d"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "SIGNING_SKEW"
kind: "static"
start_line: 38
end_line: 41
---

# SIGNING_SKEW

- Fragment ID: `rgbdns-frag-1d78c7dfd71d`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 38-41
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1d78c7dfd71d", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 38, "endLine": 41}
```

## Excerpt

<span id="rgbdns-frag-1d78c7dfd71d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1d78c7dfd71d: static SIGNING_SKEW

```rust
    static SIGNING_SKEW: Cell<u64> = const { Cell::new(0) };
}

#[derive(Clone, Debug, Eq, PartialEq)]
```
