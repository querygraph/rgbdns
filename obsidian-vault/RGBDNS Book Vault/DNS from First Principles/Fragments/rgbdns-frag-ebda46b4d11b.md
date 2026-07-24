---
type: "code-fragment"
fragment_id: "rgbdns-frag-ebda46b4d11b"
source_path: "tests/packet_properties.rs"
code_note: "DNS from First Principles/Code/tests/packet_properties.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "record"
kind: "fn"
start_line: 10
end_line: 25
---

# record

- Fragment ID: `rgbdns-frag-ebda46b4d11b`
- Source file: [[DNS from First Principles/Code/tests/packet_properties.rs.source|tests/packet_properties.rs]]
- Lines: 10-25
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-ebda46b4d11b", "codeNote": "DNS from First Principles/Code/tests/packet_properties.rs.source", "heading": "rgbdns-frag-ebda46b4d11b: fn record", "sourcePath": "tests/packet_properties.rs", "startLine": 10, "endLine": 25}
```

## Excerpt

<span id="rgbdns-frag-ebda46b4d11b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ebda46b4d11b: fn record

```rust
fn record() -> impl Strategy<Value = Record> {
    (
        dns_name(),
        any::<u32>(),
        prop_oneof![
            any::<[u8; 4]>().prop_map(|octets| RData::A(Ipv4Addr::from(octets))),
            any::<[u8; 16]>().prop_map(|octets| RData::Aaaa(Ipv6Addr::from(octets))),
            prop::collection::vec(prop::collection::vec(any::<u8>(), 0..=64), 0..=4)
                .prop_map(RData::Txt),
            prop::collection::vec(any::<u8>(), 0..=128)
                .prop_map(|bytes| RData::Opaque(RecordType::Unknown(65_000), bytes)),
        ],
    )
        .prop_map(|(name, ttl, data)| Record { name, ttl, data })
}

```
