---
type: "code-fragment"
fragment_id: "rgbdns-frag-0e6a66d6e1b2"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "empty_nonterminals_and_closest_encloser_block_higher_wildcards"
kind: "fn"
start_line: 908
end_line: 930
---

# empty_nonterminals_and_closest_encloser_block_higher_wildcards

- Fragment ID: `rgbdns-frag-0e6a66d6e1b2`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 908-930
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0e6a66d6e1b2", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-0e6a66d6e1b2: fn empty_nonterminals_and_closest_encloser_block_higher_wildcards", "sourcePath": "src/zone.rs", "startLine": 908, "endLine": 930}
```

## Excerpt

<span id="rgbdns-frag-0e6a66d6e1b2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0e6a66d6e1b2: fn empty_nonterminals_and_closest_encloser_block_higher_wildcards

```rust
    fn empty_nonterminals_and_closest_encloser_block_higher_wildcards() {
        let zone = Zone::parse(
            ".example::ns.example\n\
             +*.example:192.0.2.1\n\
             +leaf.branch.example:192.0.2.2\n",
        )
        .unwrap();
        assert!(matches!(
            zone.lookup(&"branch.example".parse().unwrap(), RecordType::A),
            Lookup::NoData(Some(_))
        ));
        assert!(matches!(
            zone.lookup(&"missing.branch.example".parse().unwrap(), RecordType::A),
            Lookup::NxDomain(Some(_))
        ));
        assert!(matches!(
            zone.lookup(&"other.example".parse().unwrap(), RecordType::A),
            Lookup::Answer(records)
                if records[0].data == RData::A(Ipv4Addr::new(192, 0, 2, 1))
        ));
    }

    #[test]
```
