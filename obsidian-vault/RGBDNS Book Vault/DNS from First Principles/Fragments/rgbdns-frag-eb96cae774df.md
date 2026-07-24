---
type: "code-fragment"
fragment_id: "rgbdns-frag-eb96cae774df"
source_path: "src/packet.rs"
code_note: "DNS from First Principles/Code/src/packet.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "parses_standard_and_generic_record_type_names"
kind: "fn"
start_line: 743
end_line: 766
---

# parses_standard_and_generic_record_type_names

- Fragment ID: `rgbdns-frag-eb96cae774df`
- Source file: [[DNS from First Principles/Code/src/packet.rs.source|src/packet.rs]]
- Lines: 743-766
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-eb96cae774df", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-eb96cae774df: fn parses_standard_and_generic_record_type_names", "sourcePath": "src/packet.rs", "startLine": 743, "endLine": 766}
```

## Excerpt

<span id="rgbdns-frag-eb96cae774df" class="rgbdns-fragment-target"></span>
### rgbdns-frag-eb96cae774df: fn parses_standard_and_generic_record_type_names

```rust
    fn parses_standard_and_generic_record_type_names() {
        for (name, code) in [
            ("HINFO", 13),
            ("RP", 17),
            ("SIG", 24),
            ("KEY", 25),
            ("NAPTR", 35),
            ("NSEC3", 50),
            ("NSEC3PARAM", 51),
            ("TLSA", 52),
            ("SVCB", 64),
            ("HTTPS", 65),
            ("TYPE65400", 65400),
            ("65401", 65401),
        ] {
            assert_eq!(
                name.parse::<RecordType>().unwrap(),
                RecordType::Unknown(code)
            );
        }
        assert!("TYPEbogus".parse::<RecordType>().is_err());
        assert!("TYPE65536".parse::<RecordType>().is_err());
    }
}
```
