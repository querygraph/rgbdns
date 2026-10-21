---
type: "code-fragment"
fragment_id: "rgbdns-frag-b342f5d5a9fa"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "signs_a_zone_with_dnskey_nsec_and_rrsigs"
kind: "fn"
start_line: 989
end_line: 1062
---

# signs_a_zone_with_dnskey_nsec_and_rrsigs

- Fragment ID: `rgbdns-frag-b342f5d5a9fa`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 989-1062
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b342f5d5a9fa", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 989, "endLine": 1062}
```

## Excerpt

<span id="rgbdns-frag-b342f5d5a9fa" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b342f5d5a9fa: fn signs_a_zone_with_dnskey_nsec_and_rrsigs

```rust
    fn signs_a_zone_with_dnskey_nsec_and_rrsigs() {
        let directory = std::env::temp_dir().join(format!(
            "rgbdns-dnssec-{}-{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        fs::create_dir_all(&directory).unwrap();
        let key = directory.join("example.pk8");
        let zone_name = "example".parse().unwrap();
        let policy = generate_key(&zone_name, &key).unwrap();
        let zone = Zone::parse(
            "Zexample:ns.example:hostmaster.example:7:3600:600:86400:300:300\n\
             &example:192.0.2.53:ns.example:300\n\
             +www.example:192.0.2.1:300\n",
        )
        .unwrap();
        let records = sign_zone(&zone, &policy).unwrap();
        assert!(
            records
                .iter()
                .any(|record| record.rr_type() == RecordType::Dnskey)
        );
        assert!(
            records
                .iter()
                .any(|record| record.rr_type() == RecordType::Nsec)
        );
        assert!(
            records
                .iter()
                .any(|record| record.rr_type() == RecordType::Rrsig)
        );
        let ds = ds_line(&policy).unwrap();
        assert!(ds.starts_with("example. IN DS "));
        assert!(ds.contains(" 13 2 "));
        let (expiration, inception) = records
            .iter()
            .find_map(|record| match record.data.dnssec().unwrap() {
                Some(DnssecRData::Rrsig {
                    expiration,
                    inception,
                    ..
                }) => Some((expiration, inception)),
                _ => None,
            })
            .unwrap();
        assert_eq!(expiration.wrapping_sub(inception), policy.validity as u32);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;
        assert!(inception <= now.saturating_sub(policy.inception_skew as u32));
        let signed = Zone::from_compiled_records(
            records
                .iter()
                .cloned()
                .map(|record| (record, crate::zone::RecordMetadata::default()))
                .collect(),
            Vec::new(),
            Vec::new(),
        );
        let status = check_zone(&signed, &policy).unwrap();
        assert_eq!(status.serial, 7);
        assert!(status.remaining >= policy.refresh);
        assert_eq!(
            records.iter().find_map(|record| match record.data {
                RData::Soa { serial, .. } => Some(serial),
                _ => None,
            }),
            Some(7)
        );
        fs::remove_dir_all(directory).unwrap();
    }
}
```
