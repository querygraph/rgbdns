---
type: "code-fragment"
fragment_id: "rgbdns-frag-c56938c18836"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "rr_type"
kind: "fn"
start_line: 204
end_line: 218
---

# rr_type

- Fragment ID: `rgbdns-frag-c56938c18836`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 204-218
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c56938c18836", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 204, "endLine": 218}
```

## Excerpt

<span id="rgbdns-frag-c56938c18836" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c56938c18836: fn rr_type

```rust
    pub fn rr_type(&self) -> RecordType {
        match self {
            Self::A(_) => RecordType::A,
            Self::Aaaa(_) => RecordType::Aaaa,
            Self::Name(t, _) => *t,
            Self::Mx(..) => RecordType::Mx,
            Self::Soa { .. } => RecordType::Soa,
            Self::Txt(_) => RecordType::Txt,
            Self::Srv { .. } => RecordType::Srv,
            Self::Caa { .. } => RecordType::Caa,
            Self::Opt { .. } => RecordType::Opt,
            Self::Opaque(t, _) => *t,
        }
    }

```
