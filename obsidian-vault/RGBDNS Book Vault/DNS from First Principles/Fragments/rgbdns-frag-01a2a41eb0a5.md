---
type: "code-fragment"
fragment_id: "rgbdns-frag-01a2a41eb0a5"
source_path: "src/bin/rgbdns-log-report.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbdns-log-report"
symbol: "accepts_ipv6_clients_and_rejects_bad_records"
kind: "fn"
start_line: 215
end_line: 226
---

# accepts_ipv6_clients_and_rejects_bad_records

- Fragment ID: `rgbdns-frag-01a2a41eb0a5`
- Source file: [[DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source|src/bin/rgbdns-log-report.rs]]
- Lines: 215-226
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbdns-log-report|rgbdns-log-report]]

```rgbdns-fragment
{"id": "rgbdns-frag-01a2a41eb0a5", "codeNote": "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source", "sourcePath": "src/bin/rgbdns-log-report.rs", "startLine": 215, "endLine": 226}
```

## Excerpt

<span id="rgbdns-frag-01a2a41eb0a5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-01a2a41eb0a5: fn accepts_ipv6_clients_and_rejects_bad_records

```rust
    fn accepts_ipv6_clients_and_rejects_bad_records() {
        let zones = vec!["example.com".to_owned()];
        let logs = concat!(
            "20010db8000000000000000000000001:0035:0001 + 0001 example.com\n",
            "nothex:0035:0001 + 0001 example.com\n",
            "7f000001:0035:0001 + 0001 outside.test\n",
        );
        let counts = aggregate(Cursor::new(logs), &zones).unwrap();
        assert_eq!(counts["example.com"].total, 1);
        assert_eq!(counts["example.com"].clients.len(), 1);
    }
}
```
