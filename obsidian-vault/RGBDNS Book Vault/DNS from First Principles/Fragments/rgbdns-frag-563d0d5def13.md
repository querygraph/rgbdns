---
type: "code-fragment"
fragment_id: "rgbdns-frag-563d0d5def13"
source_path: "src/cdb.rs"
code_note: "DNS from First Principles/Code/src/cdb.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "exact_cdb_roundtrip_preserves_lookup_semantics"
kind: "fn"
start_line: 276
end_line: 324
---

# exact_cdb_roundtrip_preserves_lookup_semantics

- Fragment ID: `rgbdns-frag-563d0d5def13`
- Source file: [[DNS from First Principles/Code/src/cdb.rs.source|src/cdb.rs]]
- Lines: 276-324
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-563d0d5def13", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-563d0d5def13: fn exact_cdb_roundtrip_preserves_lookup_semantics", "sourcePath": "src/cdb.rs", "startLine": 276, "endLine": 324}
```

## Excerpt

<span id="rgbdns-frag-563d0d5def13" class="rgbdns-fragment-target"></span>
### rgbdns-frag-563d0d5def13: fn exact_cdb_roundtrip_preserves_lookup_semantics

```rust
    fn exact_cdb_roundtrip_preserves_lookup_semantics() {
        let zone = Zone::parse(
            ".example:192.0.2.53:ns.example\n\
             +www.example:192.0.2.1:60\n\
             +*.wild.example:192.0.2.2:61\n\
             %aa:192.0.2\n\
             +located.example:192.0.2.9:64::aa\n\
             'example:hello\\072world:62\n\
             S_sip._tcp.example:192.0.2.7:sip.example:5060:10:20:63\n",
        )
        .unwrap();
        let path = std::env::temp_dir().join(format!(
            "rgbdns-cdb-{}-{}.cdb",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        compile(&zone, &path).unwrap();
        let loaded = load(&path).unwrap();
        fs::remove_file(path).unwrap();
        assert!(matches!(
            loaded.lookup(&"www.example".parse().unwrap(), RecordType::A),
            Lookup::Answer(records)
                if records[0].data == RData::A(Ipv4Addr::new(192, 0, 2, 1))
                    && records[0].ttl == 60
        ));
        assert!(matches!(
            loaded.lookup(&"x.wild.example".parse().unwrap(), RecordType::A),
            Lookup::Answer(records) if records[0].ttl == 61
        ));
        assert!(matches!(
            loaded.lookup(&"_sip._tcp.example".parse().unwrap(), RecordType::Srv),
            Lookup::Answer(_)
        ));
        assert!(matches!(
            loaded.lookup_from(
                &"located.example".parse().unwrap(),
                RecordType::A,
                std::net::IpAddr::V4(Ipv4Addr::new(192, 0, 2, 44)),
            ),
            Lookup::Answer(records)
                if records[0].data == RData::A(Ipv4Addr::new(192, 0, 2, 9))
                    && records[0].ttl == 64
        ));
    }

    #[test]
```
