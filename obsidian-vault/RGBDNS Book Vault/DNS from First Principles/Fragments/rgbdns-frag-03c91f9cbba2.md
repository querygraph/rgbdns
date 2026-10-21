---
type: "code-fragment"
fragment_id: "rgbdns-frag-03c91f9cbba2"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "follows_a_bounded_cname_chain_and_preserves_addresses"
kind: "fn"
start_line: 249
end_line: 269
---

# follows_a_bounded_cname_chain_and_preserves_addresses

- Fragment ID: `rgbdns-frag-03c91f9cbba2`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 249-269
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-03c91f9cbba2", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "sourcePath": "src/aname.rs", "startLine": 249, "endLine": 269}
```

## Excerpt

<span id="rgbdns-frag-03c91f9cbba2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-03c91f9cbba2: fn follows_a_bounded_cname_chain_and_preserves_addresses

```rust
    fn follows_a_bounded_cname_chain_and_preserves_addresses() {
        let first: Name = "one.example".parse().unwrap();
        let second: Name = "two.example".parse().unwrap();
        let answers = vec![
            Record {
                name: first.clone(),
                ttl: 90,
                data: RData::Name(RecordType::Cname, second.clone()),
            },
            Record {
                name: second,
                ttl: 60,
                data: RData::A(Ipv4Addr::new(192, 0, 2, 4)),
            },
        ];
        let (data, ttl) = addresses(&answers, &first, RecordType::A).unwrap();
        assert_eq!(data, [RData::A(Ipv4Addr::new(192, 0, 2, 4))]);
        assert_eq!(ttl, Some(60));
    }

    #[test]
```
