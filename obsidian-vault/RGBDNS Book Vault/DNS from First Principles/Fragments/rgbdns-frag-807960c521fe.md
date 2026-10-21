---
type: "code-fragment"
fragment_id: "rgbdns-frag-807960c521fe"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "wildcard"
kind: "fn"
start_line: 1081
end_line: 1088
---

# wildcard

- Fragment ID: `rgbdns-frag-807960c521fe`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 1081-1088
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-807960c521fe", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 1081, "endLine": 1088}
```

## Excerpt

<span id="rgbdns-frag-807960c521fe" class="rgbdns-fragment-target"></span>
### rgbdns-frag-807960c521fe: fn wildcard

```rust
    fn wildcard() {
        let z = Zone::parse(".example::ns.example\n+*.example:192.0.2.4\n").unwrap();
        assert!(matches!(
            z.lookup(&"x.example".parse().unwrap(), RecordType::A),
            Lookup::Answer(_)
        ))
    }
    #[test]
```
