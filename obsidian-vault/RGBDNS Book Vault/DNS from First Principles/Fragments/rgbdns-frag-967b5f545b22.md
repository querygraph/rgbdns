---
type: "code-fragment"
fragment_id: "rgbdns-frag-967b5f545b22"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "separates_address_families_and_rejects_loops"
kind: "fn"
start_line: 270
end_line: 289
---

# separates_address_families_and_rejects_loops

- Fragment ID: `rgbdns-frag-967b5f545b22`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 270-289
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-967b5f545b22", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "sourcePath": "src/aname.rs", "startLine": 270, "endLine": 289}
```

## Excerpt

<span id="rgbdns-frag-967b5f545b22" class="rgbdns-fragment-target"></span>
### rgbdns-frag-967b5f545b22: fn separates_address_families_and_rejects_loops

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

    #[test]
```
