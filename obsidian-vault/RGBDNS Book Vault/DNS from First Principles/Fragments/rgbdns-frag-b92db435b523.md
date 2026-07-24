---
type: "code-fragment"
fragment_id: "rgbdns-frag-b92db435b523"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "RecordType"
kind: "enum"
start_line: 8
end_line: 27
---

# RecordType

- Fragment ID: `rgbdns-frag-b92db435b523`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 8-27
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b92db435b523", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-b92db435b523: enum RecordType", "sourcePath": "src/packet.rs", "startLine": 8, "endLine": 27}
```

## Excerpt

<span id="rgbdns-frag-b92db435b523" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b92db435b523: enum RecordType

```rust
pub enum RecordType {
    A,
    Ns,
    Cname,
    Soa,
    Ptr,
    Mx,
    Txt,
    Aaaa,
    Srv,
    Opt,
    Caa,
    Ds,
    Rrsig,
    Nsec,
    Dnskey,
    Axfr,
    Any,
    Unknown(u16),
}
```
