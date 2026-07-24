---
type: "code-fragment"
fragment_id: "rgbdns-frag-f35c6a0bda91"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "with_wildcard"
kind: "fn"
start_line: 111
end_line: 115
---

# with_wildcard

- Fragment ID: `rgbdns-frag-f35c6a0bda91`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 111-115
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f35c6a0bda91", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-f35c6a0bda91: fn with_wildcard", "sourcePath": "src/name.rs", "startLine": 111, "endLine": 115}
```

## Excerpt

<span id="rgbdns-frag-f35c6a0bda91" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f35c6a0bda91: fn with_wildcard

```rust
    pub(crate) fn with_wildcard(&self) -> Self {
        self.wildcard()
    }
}

```
