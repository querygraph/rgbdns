---
type: "code-fragment"
fragment_id: "rgbdns-frag-d418530011a5"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "separates_address_families_and_rejects_loops"
kind: "fn"
start_line: 194
end_line: 212
---

# separates_address_families_and_rejects_loops

- Fragment ID: `rgbdns-frag-d418530011a5`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 194-212
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d418530011a5", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "heading": "rgbdns-frag-d418530011a5: fn separates_address_families_and_rejects_loops", "sourcePath": "src/aname.rs", "startLine": 194, "endLine": 212}
```

## Excerpt

<span id="rgbdns-frag-d418530011a5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d418530011a5: fn separates_address_families_and_rejects_loops

```rust
    fn separates_address_families_and_rejects_loops() {
        let name: Name = "loop.example".parse().unwrap();
        let answers = vec![
            Record {
                name: name.clone(),
                ttl: 60,
                data: RData::Aaaa(Ipv6Addr::LOCALHOST),
            },
            Record {
                name: name.clone(),
                ttl: 60,
                data: RData::Name(RecordType::Cname, name.clone()),
            },
        ];
        let (data, _) = addresses(&answers, &name, RecordType::Aaaa).unwrap();
        assert_eq!(data, [RData::Aaaa(Ipv6Addr::LOCALHOST)]);
        assert!(addresses(&answers[1..], &name, RecordType::A).is_err());
    }
}
```
