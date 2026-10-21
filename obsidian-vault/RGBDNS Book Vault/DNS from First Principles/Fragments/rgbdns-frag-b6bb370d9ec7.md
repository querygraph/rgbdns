---
type: "code-fragment"
fragment_id: "rgbdns-frag-b6bb370d9ec7"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "cname_loops_return_servfail_with_bounded_work"
kind: "fn"
start_line: 1113
end_line: 1128
---

# cname_loops_return_servfail_with_bounded_work

- Fragment ID: `rgbdns-frag-b6bb370d9ec7`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 1113-1128
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b6bb370d9ec7", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 1113, "endLine": 1128}
```

## Excerpt

<span id="rgbdns-frag-b6bb370d9ec7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b6bb370d9ec7: fn cname_loops_return_servfail_with_bounded_work

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
