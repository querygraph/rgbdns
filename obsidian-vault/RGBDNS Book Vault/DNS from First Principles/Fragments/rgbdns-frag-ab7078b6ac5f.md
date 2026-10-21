---
type: "code-fragment"
fragment_id: "rgbdns-frag-ab7078b6ac5f"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "policy_is_one_line_and_bounded"
kind: "fn"
start_line: 861
end_line: 873
---

# policy_is_one_line_and_bounded

- Fragment ID: `rgbdns-frag-ab7078b6ac5f`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 861-873
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-ab7078b6ac5f", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 861, "endLine": 873}
```

## Excerpt

<span id="rgbdns-frag-ab7078b6ac5f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ab7078b6ac5f: fn policy_is_one_line_and_bounded

```rust
    fn policy_is_one_line_and_bounded() {
        let policy = Policy::parse("Kexample:/tmp/example.pk8:13:1209600:86400:3600").unwrap();
        assert_eq!(policy.zone, "example".parse().unwrap());
        assert_eq!(
            policy.line(),
            "Kexample.:/tmp/example.pk8:13:1209600:86400:3600"
        );
        assert!(Policy::parse("Kexample:key:13:1209600:86400:3600").is_err());
        assert!(Policy::parse("Kexample:/key:15:1209600:86400:3600").is_err());
        assert!(Policy::parse("Kexample:/key:13:86400:86400:3600").is_err());
    }

    #[test]
```
