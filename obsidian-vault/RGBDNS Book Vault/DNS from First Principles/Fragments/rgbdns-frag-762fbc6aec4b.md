---
type: "code-fragment"
fragment_id: "rgbdns-frag-762fbc6aec4b"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "fmt"
kind: "fn"
start_line: 35
end_line: 43
---

# fmt

- Fragment ID: `rgbdns-frag-762fbc6aec4b`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 35-43
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-762fbc6aec4b", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-762fbc6aec4b: fn fmt", "sourcePath": "src/lib.rs", "startLine": 35, "endLine": 43}
```

## Excerpt

<span id="rgbdns-frag-762fbc6aec4b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-762fbc6aec4b: fn fmt

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
