---
type: "code-fragment"
fragment_id: "rgbdns-frag-b7c53377ea2d"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "with_wildcard"
kind: "fn"
start_line: 123
end_line: 127
---

# with_wildcard

- Fragment ID: `rgbdns-frag-b7c53377ea2d`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 123-127
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b7c53377ea2d", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "sourcePath": "src/name.rs", "startLine": 123, "endLine": 127}
```

## Excerpt

<span id="rgbdns-frag-b7c53377ea2d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b7c53377ea2d: fn with_wildcard

```rust
    pub(crate) fn with_wildcard(&self) -> Self {
        self.wildcard()
    }
}

```
