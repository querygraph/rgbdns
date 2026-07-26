---
type: "code-fragment"
fragment_id: "rgbdns-frag-ebf4f59215f2"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "Zone"
kind: "struct"
start_line: 11
end_line: 24
---

# Zone

- Fragment ID: `rgbdns-frag-ebf4f59215f2`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 11-24
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ebf4f59215f2", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-ebf4f59215f2: struct Zone", "sourcePath": "src/zone.rs", "startLine": 11, "endLine": 24}
```

## Excerpt

<span id="rgbdns-frag-ebf4f59215f2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ebf4f59215f2: struct Zone

```rust
pub struct Zone {
    records: BTreeMap<Name, Vec<Record>>,
    metadata: BTreeMap<Name, Vec<RecordMetadata>>,
    anames: BTreeMap<Name, Aname>,
    authoritative: BTreeSet<Name>,
    delegations: BTreeSet<Name>,
    locations: Vec<(Vec<u8>, [u8; 2])>,
    current_metadata: RecordMetadata,
    default_serial: u32,
    nodes: BTreeSet<Name>,
    unqualified_nodes: BTreeSet<Name>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
```
