---
type: "code-fragment"
fragment_id: "rgbdns-frag-e314247d27ae"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "code"
kind: "fn"
start_line: 29
end_line: 50
---

# code

- Fragment ID: `rgbdns-frag-e314247d27ae`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 29-50
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e314247d27ae", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-e314247d27ae: fn code", "sourcePath": "src/packet.rs", "startLine": 29, "endLine": 50}
```

## Excerpt

<span id="rgbdns-frag-e314247d27ae" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e314247d27ae: fn code

```rust
    pub fn code(self) -> u16 {
        match self {
            Self::A => 1,
            Self::Ns => 2,
            Self::Cname => 5,
            Self::Soa => 6,
            Self::Ptr => 12,
            Self::Mx => 15,
            Self::Txt => 16,
            Self::Aaaa => 28,
            Self::Srv => 33,
            Self::Opt => 41,
            Self::Ds => 43,
            Self::Rrsig => 46,
            Self::Nsec => 47,
            Self::Dnskey => 48,
            Self::Axfr => 252,
            Self::Caa => 257,
            Self::Any => 255,
            Self::Unknown(n) => n,
        }
    }
```
