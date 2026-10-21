---
type: "code-fragment"
fragment_id: "rgbdns-frag-d46e8d20b6e1"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "dnssec_is_do_gated_and_serves_signed_denial_and_ds_denial"
kind: "fn"
start_line: 818
end_line: 898
---

# dnssec_is_do_gated_and_serves_signed_denial_and_ds_denial

- Fragment ID: `rgbdns-frag-d46e8d20b6e1`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 818-898
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d46e8d20b6e1", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 818, "endLine": 898}
```

## Excerpt

<span id="rgbdns-frag-d46e8d20b6e1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d46e8d20b6e1: fn dnssec_is_do_gated_and_serves_signed_denial_and_ds_denial

```rust
    fn dnssec_is_do_gated_and_serves_signed_denial_and_ds_denial() {
        let zone = signed_test_zone();
        let unsigned = Message::decode(
            &respond(&zone, &query("www.example", RecordType::A, None), 4096).unwrap(),
        )
        .unwrap();
        assert_eq!(
            unsigned
                .answers
                .iter()
                .filter(|record| record.rr_type() == RecordType::Rrsig)
                .count(),
            0
        );

        let signed = Message::decode(
            &respond(
                &zone,
                &query("www.example", RecordType::A, Some((4096, 0))),
                4096,
            )
            .unwrap(),
        )
        .unwrap();
        assert!(
            signed
                .answers
                .iter()
                .any(|record| record.rr_type() == RecordType::A)
        );
        assert!(
            signed
                .answers
                .iter()
                .any(|record| record.rr_type() == RecordType::Rrsig)
        );

        let nonexistent = Message::decode(
            &respond(
                &zone,
                &query("missing.example", RecordType::A, Some((4096, 0))),
                4096,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(nonexistent.flags & 0x000f, 3);
        assert!(
            nonexistent
                .authorities
                .iter()
                .any(|record| record.rr_type() == RecordType::Nsec)
        );
        assert!(
            nonexistent
                .authorities
                .iter()
                .any(|record| record.rr_type() == RecordType::Rrsig)
        );

        let randomized_case = Message::decode(
            &respond(
                &zone,
                &query("MiSsInG.ExAmPlE", RecordType::A, Some((4096, 0))),
                4096,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(randomized_case.flags & 0x000f, 3);
        assert!(
            randomized_case
                .authorities
                .iter()
                .any(|record| record.rr_type() == RecordType::Nsec)
        );
        assert!(
            randomized_case
                .authorities
                .iter()
                .any(|record| record.rr_type() == RecordType::Rrsig)
```
