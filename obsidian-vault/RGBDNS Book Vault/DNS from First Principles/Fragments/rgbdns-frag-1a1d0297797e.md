---
type: "code-fragment"
fragment_id: "rgbdns-frag-1a1d0297797e"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "rr_type"
kind: "fn"
start_line: 130
end_line: 135
---

# rr_type

- Fragment ID: `rgbdns-frag-1a1d0297797e`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 130-135
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1a1d0297797e", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-1a1d0297797e: fn rr_type", "sourcePath": "src/packet.rs", "startLine": 130, "endLine": 135}
```

## Excerpt

<span id="rgbdns-frag-1a1d0297797e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1a1d0297797e: fn rr_type

```rust
    pub fn rr_type(&self) -> RecordType {
        self.data.rr_type()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
```
