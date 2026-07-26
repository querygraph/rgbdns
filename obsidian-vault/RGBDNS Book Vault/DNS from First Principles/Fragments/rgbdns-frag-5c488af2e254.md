---
type: "code-fragment"
fragment_id: "rgbdns-frag-5c488af2e254"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "RecordMetadata"
kind: "struct"
start_line: 31
end_line: 34
---

# RecordMetadata

- Fragment ID: `rgbdns-frag-5c488af2e254`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 31-34
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5c488af2e254", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-5c488af2e254: struct RecordMetadata", "sourcePath": "src/zone.rs", "startLine": 31, "endLine": 34}
```

## Excerpt

<span id="rgbdns-frag-5c488af2e254" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5c488af2e254: struct RecordMetadata

```rust
pub(crate) struct RecordMetadata {
    pub cutoff: u64,
    pub location: Option<[u8; 2]>,
}
```
