---
type: "code-fragment"
fragment_id: "rgbdns-frag-8e8684a46cf0"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "rr_type"
kind: "fn"
start_line: 172
end_line: 188
---

# rr_type

- Fragment ID: `rgbdns-frag-8e8684a46cf0`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 172-188
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-8e8684a46cf0", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-8e8684a46cf0: fn rr_type", "sourcePath": "src/packet.rs", "startLine": 172, "endLine": 188}
```

## Excerpt

<span id="rgbdns-frag-8e8684a46cf0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8e8684a46cf0: fn rr_type

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
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
```
