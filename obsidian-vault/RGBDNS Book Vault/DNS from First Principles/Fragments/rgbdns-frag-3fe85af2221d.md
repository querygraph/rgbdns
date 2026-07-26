---
type: "code-fragment"
fragment_id: "rgbdns-frag-3fe85af2221d"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "from"
kind: "fn"
start_line: 47
end_line: 50
---

# from

- Fragment ID: `rgbdns-frag-3fe85af2221d`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 47-50
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3fe85af2221d", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-3fe85af2221d: fn from", "sourcePath": "src/lib.rs", "startLine": 47, "endLine": 50}
```

## Excerpt

<span id="rgbdns-frag-3fe85af2221d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3fe85af2221d: fn from

```rust
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
```
