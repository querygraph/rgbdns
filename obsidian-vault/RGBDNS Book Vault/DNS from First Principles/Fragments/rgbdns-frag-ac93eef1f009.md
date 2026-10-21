---
type: "code-fragment"
fragment_id: "rgbdns-frag-ac93eef1f009"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "from"
kind: "fn"
start_line: 49
end_line: 52
---

# from

- Fragment ID: `rgbdns-frag-ac93eef1f009`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 49-52
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ac93eef1f009", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "sourcePath": "src/lib.rs", "startLine": 49, "endLine": 52}
```

## Excerpt

<span id="rgbdns-frag-ac93eef1f009" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ac93eef1f009: fn from

```rust
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
```
