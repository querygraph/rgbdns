---
type: "code-fragment"
fragment_id: "rgbdns-frag-02e9817d0edc"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "standard_axfr_preserves_a_finished_signed_zone"
kind: "fn"
start_line: 602
end_line: 649
---

# standard_axfr_preserves_a_finished_signed_zone

- Fragment ID: `rgbdns-frag-02e9817d0edc`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 602-649
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-02e9817d0edc", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 602, "endLine": 649}
```

## Excerpt

<span id="rgbdns-frag-02e9817d0edc" class="rgbdns-fragment-target"></span>
### rgbdns-frag-02e9817d0edc: fn standard_axfr_preserves_a_finished_signed_zone

```rust
    fn standard_axfr_preserves_a_finished_signed_zone() {
        let directory = temp_path("signed-axfr");
        fs::create_dir(&directory).unwrap();
        let key = directory.join("example.pk8");
        let apex: Name = "example".parse().unwrap();
        let policy = dnssec::generate_key(&apex, &key).unwrap();
        let source = Zone::parse(
            "Zexample:ns.example:hostmaster.example:7:3600:600:86400:300:300\n\
             &example:192.0.2.53:ns.example:300\n\
             +www.example:192.0.2.1:300\n",
        )
        .unwrap();
        let signed = Zone::from_compiled_records(
            dnssec::sign_zone(&source, &policy)
                .unwrap()
                .into_iter()
                .map(|record| (record, crate::zone::RecordMetadata::default()))
                .collect(),
            Vec::new(),
            Vec::new(),
        );
        let query = Message {
            id: 10,
            questions: vec![Question {
                name: apex.clone(),
                qtype: RecordType::Axfr,
                qclass: 1,
            }],
            ..Default::default()
        };
        let records = response_wires(&signed, query)
            .unwrap()
            .iter()
            .flat_map(|wire| Message::decode(wire).unwrap().answers)
            .collect::<Vec<_>>();
        assert_eq!(records.first(), records.last());
        for record_type in [RecordType::Dnskey, RecordType::Rrsig, RecordType::Nsec] {
            assert!(records.iter().any(|record| record.rr_type() == record_type));
        }

        let output = directory.join("data");
        write_tinydns(&records, &output, &directory.join("data.tmp")).unwrap();
        let transferred = Zone::from_file(&output).unwrap();
        dnssec::check_zone(&transferred, &policy).unwrap();
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
```
