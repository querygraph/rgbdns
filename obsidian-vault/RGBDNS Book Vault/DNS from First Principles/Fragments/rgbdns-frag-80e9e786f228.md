---
type: "code-fragment"
fragment_id: "rgbdns-frag-80e9e786f228"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "cname_loops_return_servfail_with_bounded_work"
kind: "fn"
start_line: 600
end_line: 615
---

# cname_loops_return_servfail_with_bounded_work

- Fragment ID: `rgbdns-frag-80e9e786f228`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 600-615
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-80e9e786f228", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-80e9e786f228: fn cname_loops_return_servfail_with_bounded_work", "sourcePath": "src/server.rs", "startLine": 600, "endLine": 615}
```

## Excerpt

<span id="rgbdns-frag-80e9e786f228" class="rgbdns-fragment-target"></span>
### rgbdns-frag-80e9e786f228: fn cname_loops_return_servfail_with_bounded_work

```rust
    fn cname_loops_return_servfail_with_bounded_work() {
        let zone = Zone::parse(
            ".example::ns.example\n\
             Ca.example:b.example\n\
             Cb.example:a.example\n",
        )
        .unwrap();
        let response = Message::decode(
            &respond(&zone, &query("a.example", RecordType::A, None), 4096).unwrap(),
        )
        .unwrap();
        assert_eq!(response.flags & 15, 2);
        assert!(response.answers.is_empty());
    }

    #[test]
```
