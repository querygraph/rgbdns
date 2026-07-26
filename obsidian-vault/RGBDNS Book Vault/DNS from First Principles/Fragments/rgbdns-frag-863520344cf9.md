---
type: "code-fragment"
fragment_id: "rgbdns-frag-863520344cf9"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "patched_ipv6_flat_format_has_unambiguous_ttl_and_reverse_trees"
kind: "fn"
start_line: 1029
end_line: 1052
---

# patched_ipv6_flat_format_has_unambiguous_ttl_and_reverse_trees

- Fragment ID: `rgbdns-frag-863520344cf9`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 1029-1052
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-863520344cf9", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-863520344cf9: fn patched_ipv6_flat_format_has_unambiguous_ttl_and_reverse_trees", "sourcePath": "src/zone.rs", "startLine": 1029, "endLine": 1052}
```

## Excerpt

<span id="rgbdns-frag-863520344cf9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-863520344cf9: fn patched_ipv6_flat_format_has_unambiguous_ttl_and_reverse_trees

```rust
    fn patched_ipv6_flat_format_has_unambiguous_ttl_and_reverse_trees() {
        let zone = Zone::parse(
            ".example::ns.example\n\
             6v6.example:20010db8000000000000000000000001:123\n",
        )
        .unwrap();
        assert!(matches!(
            zone.lookup(&"v6.example".parse().unwrap(), RecordType::Aaaa),
            Lookup::Answer(records)
                if records[0].ttl == 123
                    && records[0].data == RData::Aaaa("2001:db8::1".parse().unwrap())
        ));
        let nibbles = "1.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.8.b.d.0.1.0.0.2";
        for suffix in ["ip6.arpa", "ip6.int"] {
            assert!(matches!(
                zone.lookup(
                    &format!("{nibbles}.{suffix}").parse().unwrap(),
                    RecordType::Ptr,
                ),
                Lookup::Answer(_)
            ));
        }
    }
}
```
