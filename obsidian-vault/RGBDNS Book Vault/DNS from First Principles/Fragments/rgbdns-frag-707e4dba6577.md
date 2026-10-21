---
type: "code-fragment"
fragment_id: "rgbdns-frag-707e4dba6577"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "client_address_selects_tinydns_location"
kind: "fn"
start_line: 1051
end_line: 1085
---

# client_address_selects_tinydns_location

- Fragment ID: `rgbdns-frag-707e4dba6577`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 1051-1085
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-707e4dba6577", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 1051, "endLine": 1085}
```

## Excerpt

<span id="rgbdns-frag-707e4dba6577" class="rgbdns-fragment-target"></span>
### rgbdns-frag-707e4dba6577: fn client_address_selects_tinydns_location

```rust
    fn client_address_selects_tinydns_location() {
        let zone = Zone::parse(
            ".example::ns.example\n\
             %aa:192.0.2\n\
             +www.example:192.0.2.1:60::aa\n\
             +www.example:198.51.100.1:60\n",
        )
        .unwrap();
        let response = Message::decode(
            &respond_over_transport(
                &zone,
                None,
                &query("www.example", RecordType::A, None),
                4096,
                true,
                Some("192.0.2.44".parse().unwrap()),
            )
            .unwrap(),
        )
        .unwrap();
        assert!(
            response
                .answers
                .iter()
                .any(|record| record.data == RData::A("192.0.2.1".parse().unwrap()))
        );
        assert!(
            response
                .answers
                .iter()
                .any(|record| record.data == RData::A("198.51.100.1".parse().unwrap()))
        );
    }

    #[test]
```
