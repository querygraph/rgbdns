---
type: "code-fragment"
fragment_id: "rgbdns-frag-c63dc3bb3049"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "query_logging_defaults_on_and_has_an_explicit_opt_out"
kind: "fn"
start_line: 990
end_line: 1001
---

# query_logging_defaults_on_and_has_an_explicit_opt_out

- Fragment ID: `rgbdns-frag-c63dc3bb3049`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 990-1001
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c63dc3bb3049", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 990, "endLine": 1001}
```

## Excerpt

<span id="rgbdns-frag-c63dc3bb3049" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c63dc3bb3049: fn query_logging_defaults_on_and_has_an_explicit_opt_out

```rust
    fn query_logging_defaults_on_and_has_an_explicit_opt_out() {
        assert!(parse_query_log(None).unwrap());
        for value in ["1", "true", "YES", "on"] {
            assert!(parse_query_log(Some(value)).unwrap());
        }
        for value in ["0", "false", "NO", "off"] {
            assert!(!parse_query_log(Some(value)).unwrap());
        }
        assert!(parse_query_log(Some("sometimes")).is_err());
    }

    #[test]
```
