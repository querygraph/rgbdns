---
type: "code-fragment"
fragment_id: "rgbdns-frag-23d05c9e12ba"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "aname_coexists_with_apex_authority_but_not_address_or_cname_data"
kind: "fn"
start_line: 1297
end_line: 1324
---

# aname_coexists_with_apex_authority_but_not_address_or_cname_data

- Fragment ID: `rgbdns-frag-23d05c9e12ba`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 1297-1324
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-23d05c9e12ba", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 1297, "endLine": 1324}
```

## Excerpt

<span id="rgbdns-frag-23d05c9e12ba" class="rgbdns-fragment-target"></span>
### rgbdns-frag-23d05c9e12ba: fn aname_coexists_with_apex_authority_but_not_address_or_cname_data

```rust
    fn aname_coexists_with_apex_authority_but_not_address_or_cname_data() {
        let zone = Zone::parse(
            ".example:192.0.2.53:ns.example\n\
             Aexample:hosting.example.net:120\n",
        )
        .unwrap();
        let aname = zone.aname(&"example".parse().unwrap()).unwrap();
        assert_eq!(aname.target, "hosting.example.net".parse().unwrap());
        assert_eq!(aname.ttl, 120);
        assert!(
            Zone::parse(
                ".example:192.0.2.53:ns.example\n\
                 Aexample:hosting.example.net\n\
                 +example:192.0.2.1\n",
            )
            .is_err()
        );
        assert!(
            Zone::parse(
                ".example:192.0.2.53:ns.example\n\
                 Aexample:hosting.example.net\n\
                 Cexample:other.example.net\n",
            )
            .is_err()
        );
    }

    #[test]
```
