---
type: "code-fragment"
fragment_id: "rgbdns-frag-b839244ad944"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "any_class_retains_original_tinydns_behavior"
kind: "fn"
start_line: 1002
end_line: 1012
---

# any_class_retains_original_tinydns_behavior

- Fragment ID: `rgbdns-frag-b839244ad944`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 1002-1012
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b839244ad944", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 1002, "endLine": 1012}
```

## Excerpt

<span id="rgbdns-frag-b839244ad944" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b839244ad944: fn any_class_retains_original_tinydns_behavior

```rust
    fn any_class_retains_original_tinydns_behavior() {
        let zone = Zone::parse(".example::ns.example\n+www.example:192.0.2.1\n").unwrap();
        let mut request = Message::decode(&query("www.example", RecordType::A, None)).unwrap();
        request.questions[0].qclass = 255;
        let response =
            Message::decode(&respond(&zone, &request.encode().unwrap(), 4096).unwrap()).unwrap();
        assert_eq!(response.flags & 0x000f, 0);
        assert_eq!(response.answers.len(), 1);
    }

    #[test]
```
