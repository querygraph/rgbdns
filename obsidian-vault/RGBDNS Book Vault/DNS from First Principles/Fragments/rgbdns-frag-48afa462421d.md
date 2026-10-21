---
type: "code-fragment"
fragment_id: "rgbdns-frag-48afa462421d"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "query_log_names_cannot_inject_lines"
kind: "fn"
start_line: 980
end_line: 989
---

# query_log_names_cannot_inject_lines

- Fragment ID: `rgbdns-frag-48afa462421d`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 980-989
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-48afa462421d", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 980, "endLine": 989}
```

## Excerpt

<span id="rgbdns-frag-48afa462421d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-48afa462421d: fn query_log_names_cannot_inject_lines

```rust
    fn query_log_names_cannot_inject_lines() {
        let peer = "127.0.0.1:53".parse().unwrap();
        let request = query("line\\010break.example", RecordType::Txt, None);
        let line = query_log_line(peer, &request, None);
        assert_eq!(line, "7f000001:0035:1234 + 0010 line\\010break.example");
        assert!(!line.contains('\n'));
        assert!(!line.contains('\r'));
    }

    #[test]
```
