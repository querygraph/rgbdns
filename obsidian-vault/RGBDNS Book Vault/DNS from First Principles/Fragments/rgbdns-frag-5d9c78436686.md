---
type: "code-fragment"
fragment_id: "rgbdns-frag-5d9c78436686"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "live_transfer_has_matching_soa_bookends"
kind: "fn"
start_line: 387
end_line: 414
---

# live_transfer_has_matching_soa_bookends

- Fragment ID: `rgbdns-frag-5d9c78436686`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 387-414
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5d9c78436686", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-5d9c78436686: fn live_transfer_has_matching_soa_bookends", "sourcePath": "src/axfr.rs", "startLine": 387, "endLine": 414}
```

## Excerpt

<span id="rgbdns-frag-5d9c78436686" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5d9c78436686: fn live_transfer_has_matching_soa_bookends

```rust
    fn live_transfer_has_matching_soa_bookends() {
        let zone = Zone::parse(
            "Zexample:ns.example:hostmaster.example:7:8:9:10:11:12\n\
             &example:192.0.2.53:ns.example:300\n\
             +www.example:192.0.2.1:60\n",
        )
        .unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        thread::spawn(move || {
            serve_listener(
                Arc::new(zone),
                listener,
                Arc::new(vec!["127.0.0.0/8".parse().unwrap()]),
            )
            .unwrap();
        });
        let records = fetch(address, "example".parse().unwrap()).unwrap();
        assert!(records.len() >= 4);
        assert_eq!(records.first(), records.last());
        assert_eq!(records.first().unwrap().rr_type(), RecordType::Soa);
        assert!(records.iter().any(|record| {
            record.name == "www.example".parse().unwrap()
                && record.data == RData::A(Ipv4Addr::new(192, 0, 2, 1))
        }));
    }

    #[test]
```
