---
type: "code-fragment"
fragment_id: "rgbdns-frag-b5ef588ef89f"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "malformed_queries_get_bounded_formerr_and_unknown_opcode_gets_notimp"
kind: "fn"
start_line: 507
end_line: 532
---

# malformed_queries_get_bounded_formerr_and_unknown_opcode_gets_notimp

- Fragment ID: `rgbdns-frag-b5ef588ef89f`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 507-532
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b5ef588ef89f", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-b5ef588ef89f: fn malformed_queries_get_bounded_formerr_and_unknown_opcode_gets_notimp", "sourcePath": "src/server.rs", "startLine": 507, "endLine": 532}
```

## Excerpt

<span id="rgbdns-frag-b5ef588ef89f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b5ef588ef89f: fn malformed_queries_get_bounded_formerr_and_unknown_opcode_gets_notimp

```rust
    fn malformed_queries_get_bounded_formerr_and_unknown_opcode_gets_notimp() {
        let zone = Zone::parse(".example::ns.example\n").unwrap();
        let mut malformed = query("example", RecordType::A, None);
        malformed[5] = 2;
        let response = Message::decode(&respond(&zone, &malformed, 4096).unwrap()).unwrap();
        assert_eq!(response.flags & 15, 1);
        assert!(response.questions.is_empty());
        assert_eq!(response.encode().unwrap().len(), 12);

        let mut opcode = query("example", RecordType::A, None);
        opcode[2] |= 0x08;
        let response = Message::decode(&respond(&zone, &opcode, 4096).unwrap()).unwrap();
        assert_eq!(response.flags & 15, 4);

        let mut duplicate_opt =
            Message::decode(&query("example", RecordType::A, Some((1232, 0)))).unwrap();
        duplicate_opt
            .additionals
            .push(duplicate_opt.additionals[0].clone());
        let response =
            Message::decode(&respond(&zone, &duplicate_opt.encode().unwrap(), 4096).unwrap())
                .unwrap();
        assert_eq!(response.flags & 15, 1);
    }

    #[test]
```
