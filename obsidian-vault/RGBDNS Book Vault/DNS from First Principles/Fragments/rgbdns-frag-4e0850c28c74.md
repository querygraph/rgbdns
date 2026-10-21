---
type: "code-fragment"
fragment_id: "rgbdns-frag-4e0850c28c74"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "mixed_snapshot_signs_k_zones_and_preserves_u_zone_aname"
kind: "fn"
start_line: 905
end_line: 985
---

# mixed_snapshot_signs_k_zones_and_preserves_u_zone_aname

- Fragment ID: `rgbdns-frag-4e0850c28c74`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 905-985
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4e0850c28c74", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 905, "endLine": 985}
```

## Excerpt

<span id="rgbdns-frag-4e0850c28c74" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4e0850c28c74: fn mixed_snapshot_signs_k_zones_and_preserves_u_zone_aname

```rust
    fn mixed_snapshot_signs_k_zones_and_preserves_u_zone_aname() {
        let directory = std::env::temp_dir().join(format!(
            "rgbdns-dnssec-mixed-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&directory).unwrap();
        let signed_name: Name = "signed.example".parse().unwrap();
        let unsigned_name: Name = "legacy.example".parse().unwrap();
        let policy = generate_key(&signed_name, &directory.join("signed.pk8")).unwrap();
        let policies = PolicySet {
            signing: vec![policy.clone()],
            unsigned: BTreeSet::from([unsigned_name.clone()]),
        };
        let source = Zone::parse(
            "Zsigned.example:ns.signed.example:hostmaster.signed.example:7:3600:600:86400:300:300\n\
             &signed.example:192.0.2.53:ns.signed.example:300\n\
             +www.signed.example:192.0.2.1:300\n\
             Zlegacy.example:ns.legacy.example:hostmaster.legacy.example:8:3600:600:86400:300:300\n\
             &legacy.example:192.0.2.54:ns.legacy.example:300\n\
             Alegacy.example:target.example:120\n\
             %aa:192.0.2\n\
             +geo.legacy.example:192.0.2.9:60::aa\n\
             +geo.legacy.example:198.51.100.9:60\n",
        )
        .unwrap();
        let snapshot = build_snapshot(&source, &policies).unwrap();
        assert!(snapshot.is_dnssec_signed(&signed_name));
        assert!(!snapshot.is_dnssec_signed(&unsigned_name));
        assert_eq!(
            snapshot.aname(&unsigned_name).unwrap().target,
            "target.example".parse().unwrap()
        );
        assert!(matches!(
            snapshot.lookup_from(
                &"geo.legacy.example".parse().unwrap(),
                RecordType::A,
                "192.0.2.44".parse().unwrap()
            ),
            crate::zone::Lookup::Answer(records)
                if records.iter().any(|record| record.data == RData::A("192.0.2.9".parse().unwrap()))
        ));
        check_zone(&snapshot, &policy).unwrap();

        let source_file = directory.join("data");
        let policy_file = directory.join("dnssec");
        let materialized = directory.join("data.materialized");
        fs::write(
            &source_file,
            "Zsigned.example:ns.signed.example:hostmaster.signed.example:7:3600:600:86400:300:300\n\
             &signed.example:192.0.2.53:ns.signed.example:300\n\
             Zlegacy.example:ns.legacy.example:hostmaster.legacy.example:8:3600:600:86400:300:300\n\
             &legacy.example:192.0.2.54:ns.legacy.example:300\n\
             Alegacy.example:target.example:120\n\
             %aa:192.0.2\n\
             +geo.legacy.example:192.0.2.9:60::aa\n",
        )
        .unwrap();
        fs::write(
            &policy_file,
            format!("{}\nUlegacy.example.\n", policies.signing[0].line()),
        )
        .unwrap();
        assert_eq!(
            materialize_policy_file(&source_file, &policy_file, &materialized).unwrap(),
            None
        );
        assert_eq!(
            fs::read(&source_file).unwrap(),
            fs::read(&materialized).unwrap()
        );

        let incomplete = PolicySet {
            signing: vec![policy],
            unsigned: BTreeSet::new(),
        };
        assert!(build_snapshot(&source, &incomplete).is_err());
        fs::remove_dir_all(directory).unwrap();
```
