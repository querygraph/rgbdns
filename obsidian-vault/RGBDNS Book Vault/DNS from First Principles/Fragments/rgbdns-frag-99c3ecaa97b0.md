---
type: "code-fragment"
fragment_id: "rgbdns-frag-99c3ecaa97b0"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "query_logs_cover_malformed_unimplemented_class_ipv6_and_axfr"
kind: "fn"
start_line: 948
end_line: 979
---

# query_logs_cover_malformed_unimplemented_class_ipv6_and_axfr

- Fragment ID: `rgbdns-frag-99c3ecaa97b0`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 948-979
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-99c3ecaa97b0", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 948, "endLine": 979}
```

## Excerpt

<span id="rgbdns-frag-99c3ecaa97b0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-99c3ecaa97b0: fn query_logs_cover_malformed_unimplemented_class_ipv6_and_axfr

```rust
    fn query_logs_cover_malformed_unimplemented_class_ipv6_and_axfr() {
        let ipv4 = "192.0.2.1:53".parse().unwrap();
        assert_eq!(
            query_log_line(ipv4, &[0, 1, 2], None),
            "c0000201:0035:0000 / 0000 ."
        );

        let mut unimplemented = Message::decode(&query("example", RecordType::Aaaa, None)).unwrap();
        unimplemented.flags |= 0x0800;
        let unimplemented = unimplemented.encode().unwrap();
        assert_eq!(
            query_log_line(ipv4, &unimplemented, None),
            "c0000201:0035:1234 I 001c example"
        );

        let mut other_class = Message::decode(&query("example", RecordType::A, None)).unwrap();
        other_class.questions[0].qclass = 3;
        let other_class = other_class.encode().unwrap();
        assert_eq!(
            query_log_line(ipv4, &other_class, None),
            "c0000201:0035:1234 C 0001 example"
        );

        let ipv6 = "[2001:db8::1]:4660".parse().unwrap();
        let axfr = query("example", RecordType::Axfr, None);
        assert_eq!(
            query_log_line_with_code(ipv6, &axfr, '+'),
            "20010db8000000000000000000000001:1234:1234 + 00fc example"
        );
    }

    #[test]
```
