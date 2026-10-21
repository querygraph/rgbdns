---
type: "code-fragment"
fragment_id: "rgbdns-frag-19e36617d117"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "malformed_negotiated_aname_is_not_installed"
kind: "fn"
start_line: 675
end_line: 690
---

# malformed_negotiated_aname_is_not_installed

- Fragment ID: `rgbdns-frag-19e36617d117`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 675-690
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-19e36617d117", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 675, "endLine": 690}
```

## Excerpt

<span id="rgbdns-frag-19e36617d117" class="rgbdns-fragment-target"></span>
### rgbdns-frag-19e36617d117: fn malformed_negotiated_aname_is_not_installed

```rust
    fn malformed_negotiated_aname_is_not_installed() {
        let output = temp_path("bad-aname-data");
        let temporary = temp_path("bad-aname-data-tmp");
        let mut payload = RGBDNS_ANAME_MAGIC.to_vec();
        payload.extend([10, b'x']);
        let records = vec![Record {
            name: "example".parse().unwrap(),
            ttl: 300,
            data: RData::Opaque(RecordType::Unknown(RGBDNS_ANAME_RECORD), payload),
        }];
        assert!(write_tinydns(&records, &output, &temporary).is_err());
        assert!(!temporary.exists());
        assert!(!output.exists());
    }

    #[test]
```
