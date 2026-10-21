---
type: "code-fragment"
fragment_id: "rgbdns-frag-bf9ff37de005"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "dnssec_rdata_is_typed_and_malformed_values_are_rejected"
kind: "fn"
start_line: 858
end_line: 898
---

# dnssec_rdata_is_typed_and_malformed_values_are_rejected

- Fragment ID: `rgbdns-frag-bf9ff37de005`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 858-898
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-bf9ff37de005", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "sourcePath": "src/packet.rs", "startLine": 858, "endLine": 898}
```

## Excerpt

<span id="rgbdns-frag-bf9ff37de005" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bf9ff37de005: fn dnssec_rdata_is_typed_and_malformed_values_are_rejected

```rust
    fn dnssec_rdata_is_typed_and_malformed_values_are_rejected() {
        assert!(
            RData::Opaque(RecordType::Dnskey, vec![1, 0, 2, 13, 1])
                .dnssec()
                .is_err()
        );
        assert!(RData::Opaque(RecordType::Ds, vec![0; 4]).dnssec().is_err());
        assert!(
            RData::Opaque(RecordType::Rrsig, vec![0; 18])
                .dnssec()
                .is_err()
        );
        assert!(
            RData::Opaque(RecordType::Nsec, vec![0, 0, 1, 0])
                .dnssec()
                .is_err()
        );

        assert_eq!(
            RData::Opaque(RecordType::Dnskey, vec![1, 1, 3, 13, 7])
                .dnssec()
                .unwrap(),
            Some(DnssecRData::Dnskey {
                flags: 257,
                protocol: 3,
                algorithm: 13,
                public_key: vec![7],
            })
        );
        assert_eq!(
            RData::Opaque(RecordType::Nsec, vec![0, 0, 1, 0x40])
                .dnssec()
                .unwrap(),
            Some(DnssecRData::Nsec {
                next: Name::root(),
                types: vec![RecordType::A],
            })
        );
    }

    #[test]
```
