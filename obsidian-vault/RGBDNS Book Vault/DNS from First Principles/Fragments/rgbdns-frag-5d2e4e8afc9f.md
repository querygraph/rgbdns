---
type: "code-fragment"
fragment_id: "rgbdns-frag-5d2e4e8afc9f"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "expands_bounded_cname_chains_and_target_additionals"
kind: "fn"
start_line: 1086
end_line: 1112
---

# expands_bounded_cname_chains_and_target_additionals

- Fragment ID: `rgbdns-frag-5d2e4e8afc9f`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 1086-1112
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5d2e4e8afc9f", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 1086, "endLine": 1112}
```

## Excerpt

<span id="rgbdns-frag-5d2e4e8afc9f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5d2e4e8afc9f: fn expands_bounded_cname_chains_and_target_additionals

```rust
    fn expands_bounded_cname_chains_and_target_additionals() {
        let zone = Zone::parse(
            ".example::ns.example\n\
             Calias.example:middle.example\n\
             Cmiddle.example:www.example\n\
             +www.example:192.0.2.1:60\n\
             @example:192.0.2.25:mail.example:10:300\n",
        )
        .unwrap();
        let cname = Message::decode(
            &respond(&zone, &query("alias.example", RecordType::A, None), 4096).unwrap(),
        )
        .unwrap();
        assert_eq!(cname.answers.len(), 3);
        assert_eq!(cname.answers[2].rr_type(), RecordType::A);
        let mx = Message::decode(
            &respond(&zone, &query("example", RecordType::Mx, None), 4096).unwrap(),
        )
        .unwrap();
        assert!(
            mx.additionals
                .iter()
                .any(|record| record.data == RData::A("192.0.2.25".parse().unwrap()))
        );
    }

    #[test]
```
