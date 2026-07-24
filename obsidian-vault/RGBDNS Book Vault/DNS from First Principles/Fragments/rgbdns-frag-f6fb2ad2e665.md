---
type: "code-fragment"
fragment_id: "rgbdns-frag-f6fb2ad2e665"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "exported_tinydns_text_roundtrips"
kind: "fn"
start_line: 398
end_line: 422
---

# exported_tinydns_text_roundtrips

- Fragment ID: `rgbdns-frag-f6fb2ad2e665`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 398-422
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f6fb2ad2e665", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-f6fb2ad2e665: fn exported_tinydns_text_roundtrips", "sourcePath": "src/axfr.rs", "startLine": 398, "endLine": 422}
```

## Excerpt

<span id="rgbdns-frag-f6fb2ad2e665" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f6fb2ad2e665: fn exported_tinydns_text_roundtrips

```rust
    fn exported_tinydns_text_roundtrips() {
        let source = Zone::parse(
            "Zexample:ns.example:hostmaster.example:7:8:9:10:11:12\n\
             &example:192.0.2.53:ns.example:300\n\
             @example:192.0.2.25:mail.example:20:400\n\
             'example:hello\\072world:64\n",
        )
        .unwrap();
        let records = source.transfer(&"example".parse().unwrap()).unwrap();
        let output = temp_path("axfr-data");
        let temporary = temp_path("axfr-data-tmp");
        write_tinydns(&records, &output, &temporary).unwrap();
        let imported = Zone::from_file(&output).unwrap();
        fs::remove_file(output).unwrap();
        assert!(matches!(
            imported.lookup(&"example".parse().unwrap(), RecordType::Mx),
            Lookup::Answer(records) if matches!(records[0].data, RData::Mx(20, _))
        ));
        assert!(matches!(
            imported.lookup(&"example".parse().unwrap(), RecordType::Txt),
            Lookup::Answer(_)
        ));
    }

    #[test]
```
