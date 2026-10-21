---
type: "code-fragment"
fragment_id: "rgbdns-frag-bb1fd64a4514"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "wire_len"
kind: "fn"
start_line: 102
end_line: 104
---

# wire_len

- Fragment ID: `rgbdns-frag-bb1fd64a4514`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 102-104
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-bb1fd64a4514", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "sourcePath": "src/name.rs", "startLine": 102, "endLine": 104}
```

## Excerpt

<span id="rgbdns-frag-bb1fd64a4514" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bb1fd64a4514: fn wire_len

```rust
    pub(crate) fn wire_len(&self) -> usize {
        1 + self.0.iter().map(|l| l.len() + 1).sum::<usize>()
    }
```
