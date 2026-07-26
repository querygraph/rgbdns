---
type: "code-fragment"
fragment_id: "rgbdns-frag-138ac655395d"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "referral_has_no_aa_and_contains_bailiwick_glue"
kind: "fn"
start_line: 515
end_line: 537
---

# referral_has_no_aa_and_contains_bailiwick_glue

- Fragment ID: `rgbdns-frag-138ac655395d`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 515-537
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-138ac655395d", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-138ac655395d: fn referral_has_no_aa_and_contains_bailiwick_glue", "sourcePath": "src/server.rs", "startLine": 515, "endLine": 537}
```

## Excerpt

<span id="rgbdns-frag-138ac655395d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-138ac655395d: fn referral_has_no_aa_and_contains_bailiwick_glue

```rust
    fn referral_has_no_aa_and_contains_bailiwick_glue() {
        let zone = Zone::parse(".example::ns.example\n&child.example:192.0.2.2:ns.child.example\n")
            .unwrap();
        let response = Message::decode(
            &respond(
                &zone,
                &query("host.child.example", RecordType::A, None),
                4096,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(response.flags & 0x0400, 0);
        assert_eq!(response.authorities.len(), 1);
        assert!(
            response
                .additionals
                .iter()
                .any(|record| record.rr_type() == RecordType::A)
        );
    }

    #[test]
```
