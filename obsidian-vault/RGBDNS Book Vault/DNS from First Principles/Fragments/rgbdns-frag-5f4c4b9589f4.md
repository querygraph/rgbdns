---
type: "code-fragment"
fragment_id: "rgbdns-frag-5f4c4b9589f4"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "partial_cmp"
kind: "fn"
start_line: 45
end_line: 49
---

# partial_cmp

- Fragment ID: `rgbdns-frag-5f4c4b9589f4`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 45-49
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5f4c4b9589f4", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-5f4c4b9589f4: fn partial_cmp", "sourcePath": "src/name.rs", "startLine": 45, "endLine": 49}
```

## Excerpt

<span id="rgbdns-frag-5f4c4b9589f4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5f4c4b9589f4: fn partial_cmp

```rust
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

```
