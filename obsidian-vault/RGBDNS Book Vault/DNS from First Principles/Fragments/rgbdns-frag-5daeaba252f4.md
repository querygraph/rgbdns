---
type: "code-fragment"
fragment_id: "rgbdns-frag-5daeaba252f4"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "Zone"
kind: "struct"
start_line: 11
end_line: 23
---

# Zone

- Fragment ID: `rgbdns-frag-5daeaba252f4`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 11-23
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5daeaba252f4", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-5daeaba252f4: struct Zone", "sourcePath": "src/zone.rs", "startLine": 11, "endLine": 23}
```

## Excerpt

<span id="rgbdns-frag-5daeaba252f4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5daeaba252f4: struct Zone

```rust
pub struct Zone {
    records: BTreeMap<Name, Vec<Record>>,
    metadata: BTreeMap<Name, Vec<RecordMetadata>>,
    authoritative: BTreeSet<Name>,
    delegations: BTreeSet<Name>,
    locations: Vec<(Vec<u8>, [u8; 2])>,
    current_metadata: RecordMetadata,
    default_serial: u32,
    nodes: BTreeSet<Name>,
    unqualified_nodes: BTreeSet<Name>,
}

#[derive(Clone, Copy, Debug, Default)]
```
