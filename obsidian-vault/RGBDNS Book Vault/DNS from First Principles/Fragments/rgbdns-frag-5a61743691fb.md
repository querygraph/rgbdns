---
type: "code-fragment"
fragment_id: "rgbdns-frag-5a61743691fb"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "DnssecRData"
kind: "enum"
start_line: 173
end_line: 202
---

# DnssecRData

- Fragment ID: `rgbdns-frag-5a61743691fb`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 173-202
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5a61743691fb", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 173, "endLine": 202}
```

## Excerpt

<span id="rgbdns-frag-5a61743691fb" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5a61743691fb: enum DnssecRData

```rust
pub enum DnssecRData {
    Ds {
        key_tag: u16,
        algorithm: u8,
        digest_type: u8,
        digest: Vec<u8>,
    },
    Rrsig {
        type_covered: RecordType,
        algorithm: u8,
        labels: u8,
        original_ttl: u32,
        expiration: u32,
        inception: u32,
        key_tag: u16,
        signer: Name,
        signature: Vec<u8>,
    },
    Nsec {
        next: Name,
        types: Vec<RecordType>,
    },
    Dnskey {
        flags: u16,
        protocol: u8,
        algorithm: u8,
        public_key: Vec<u8>,
    },
}

```
