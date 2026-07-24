---
type: "code-fragment"
fragment_id: "rgbdns-frag-c25798b08316"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "from"
kind: "fn"
start_line: 46
end_line: 49
---

# from

- Fragment ID: `rgbdns-frag-c25798b08316`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 46-49
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c25798b08316", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-c25798b08316: fn from", "sourcePath": "src/lib.rs", "startLine": 46, "endLine": 49}
```

## Excerpt

<span id="rgbdns-frag-c25798b08316" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c25798b08316: fn from

```rust
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
```
