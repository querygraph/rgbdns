---
type: "code-fragment"
fragment_id: "rgbdns-frag-dcb0e69b0619"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "query_logs_match_the_original_tinydns_ipv4_format"
kind: "fn"
start_line: 921
end_line: 947
---

# query_logs_match_the_original_tinydns_ipv4_format

- Fragment ID: `rgbdns-frag-dcb0e69b0619`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 921-947
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-dcb0e69b0619", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 921, "endLine": 947}
```

## Excerpt

<span id="rgbdns-frag-dcb0e69b0619" class="rgbdns-fragment-target"></span>
### rgbdns-frag-dcb0e69b0619: fn query_logs_match_the_original_tinydns_ipv4_format

```rust
    fn query_logs_match_the_original_tinydns_ipv4_format() {
        let peer = "127.0.0.1:57876".parse().unwrap();
        let request = query("fieldnotes.es", RecordType::A, None);
        let response = respond(
            &Zone::parse(".fieldnotes.es::a.ns.cron.sh\n").unwrap(),
            &request,
            4096,
        )
        .unwrap();
        assert_eq!(
            query_log_line(peer, &request, Some(&response)),
            "7f000001:e214:1234 + 0001 fieldnotes.es"
        );

        let refused = respond(
            &Zone::parse(".example::ns.example\n").unwrap(),
            &request,
            4096,
        )
        .unwrap();
        assert_eq!(
            query_log_line(peer, &request, Some(&refused)),
            "7f000001:e214:1234 - 0001 fieldnotes.es"
        );
    }

    #[test]
```
