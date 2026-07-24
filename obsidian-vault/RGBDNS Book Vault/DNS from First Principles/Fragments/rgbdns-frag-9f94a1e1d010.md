---
type: "code-fragment"
fragment_id: "rgbdns-frag-9f94a1e1d010"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "from_code"
kind: "fn"
start_line: 51
end_line: 73
---

# from_code

- Fragment ID: `rgbdns-frag-9f94a1e1d010`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 51-73
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9f94a1e1d010", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-9f94a1e1d010: fn from_code", "sourcePath": "src/packet.rs", "startLine": 51, "endLine": 73}
```

## Excerpt

<span id="rgbdns-frag-9f94a1e1d010" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9f94a1e1d010: fn from_code

```rust
    pub fn from_code(n: u16) -> Self {
        match n {
            1 => Self::A,
            2 => Self::Ns,
            5 => Self::Cname,
            6 => Self::Soa,
            12 => Self::Ptr,
            15 => Self::Mx,
            16 => Self::Txt,
            28 => Self::Aaaa,
            33 => Self::Srv,
            41 => Self::Opt,
            43 => Self::Ds,
            46 => Self::Rrsig,
            47 => Self::Nsec,
            48 => Self::Dnskey,
            252 => Self::Axfr,
            257 => Self::Caa,
            255 => Self::Any,
            n => Self::Unknown(n),
        }
    }
}
```
