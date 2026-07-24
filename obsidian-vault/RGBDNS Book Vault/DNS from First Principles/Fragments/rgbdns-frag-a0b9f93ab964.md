---
type: "code-fragment"
fragment_id: "rgbdns-frag-a0b9f93ab964"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "RecordMetadata"
kind: "struct"
start_line: 24
end_line: 27
---

# RecordMetadata

- Fragment ID: `rgbdns-frag-a0b9f93ab964`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 24-27
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a0b9f93ab964", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-a0b9f93ab964: struct RecordMetadata", "sourcePath": "src/zone.rs", "startLine": 24, "endLine": 27}
```

## Excerpt

<span id="rgbdns-frag-a0b9f93ab964" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a0b9f93ab964: struct RecordMetadata

```rust
pub(crate) struct RecordMetadata {
    pub cutoff: u64,
    pub location: Option<[u8; 2]>,
}
```
