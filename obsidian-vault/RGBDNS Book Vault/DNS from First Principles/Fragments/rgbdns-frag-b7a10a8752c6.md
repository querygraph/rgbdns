---
type: "code-fragment"
fragment_id: "rgbdns-frag-b7a10a8752c6"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "fmt"
kind: "fn"
start_line: 36
end_line: 44
---

# fmt

- Fragment ID: `rgbdns-frag-b7a10a8752c6`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 36-44
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b7a10a8752c6", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-b7a10a8752c6: fn fmt", "sourcePath": "src/lib.rs", "startLine": 36, "endLine": 44}
```

## Excerpt

<span id="rgbdns-frag-b7a10a8752c6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b7a10a8752c6: fn fmt

```rust
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Format(s) => write!(f, "DNS format error: {s}"),
            Self::InvalidName(s) => write!(f, "invalid DNS name: {s}"),
            Self::InvalidRecord(s) => write!(f, "invalid tinydns record: {s}"),
        }
    }
}
```
