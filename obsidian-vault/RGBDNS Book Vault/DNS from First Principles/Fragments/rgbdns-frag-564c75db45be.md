---
type: "code-fragment"
fragment_id: "rgbdns-frag-564c75db45be"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "expands_bounded_cname_chains_and_target_additionals"
kind: "fn"
start_line: 464
end_line: 490
---

# expands_bounded_cname_chains_and_target_additionals

- Fragment ID: `rgbdns-frag-564c75db45be`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 464-490
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-564c75db45be", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-564c75db45be: fn expands_bounded_cname_chains_and_target_additionals", "sourcePath": "src/server.rs", "startLine": 464, "endLine": 490}
```

## Excerpt

<span id="rgbdns-frag-564c75db45be" class="rgbdns-fragment-target"></span>
### rgbdns-frag-564c75db45be: fn expands_bounded_cname_chains_and_target_additionals

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
