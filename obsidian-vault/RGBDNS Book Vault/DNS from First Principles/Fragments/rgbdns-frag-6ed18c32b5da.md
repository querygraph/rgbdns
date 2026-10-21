---
type: "code-fragment"
fragment_id: "rgbdns-frag-6ed18c32b5da"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "fmt"
kind: "fn"
start_line: 38
end_line: 46
---

# fmt

- Fragment ID: `rgbdns-frag-6ed18c32b5da`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 38-46
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-6ed18c32b5da", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "sourcePath": "src/lib.rs", "startLine": 38, "endLine": 46}
```

## Excerpt

<span id="rgbdns-frag-6ed18c32b5da" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6ed18c32b5da: fn fmt

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
