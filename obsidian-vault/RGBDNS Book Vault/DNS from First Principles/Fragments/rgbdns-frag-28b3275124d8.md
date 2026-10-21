---
type: "code-fragment"
fragment_id: "rgbdns-frag-28b3275124d8"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "RData"
kind: "enum"
start_line: 136
end_line: 172
---

# RData

- Fragment ID: `rgbdns-frag-28b3275124d8`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 136-172
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-28b3275124d8", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 136, "endLine": 172}
```

## Excerpt

<span id="rgbdns-frag-28b3275124d8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-28b3275124d8: enum RData

```rust
pub enum RData {
    A(Ipv4Addr),
    Aaaa(Ipv6Addr),
    Name(RecordType, Name),
    Mx(u16, Name),
    Soa {
        mname: Name,
        admin: Name,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum: u32,
    },
    Txt(Vec<Vec<u8>>),
    Srv {
        priority: u16,
        weight: u16,
        port: u16,
        target: Name,
    },
    Caa {
        flags: u8,
        tag: Vec<u8>,
        value: Vec<u8>,
    },
    Opt {
        udp_payload: u16,
        extended_rcode: u8,
        version: u8,
        flags: u16,
        options: Vec<u8>,
    },
    Opaque(RecordType, Vec<u8>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
```
