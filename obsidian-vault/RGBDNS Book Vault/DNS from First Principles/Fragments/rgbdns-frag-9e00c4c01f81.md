---
type: "code-fragment"
fragment_id: "rgbdns-frag-9e00c4c01f81"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "wildcard"
kind: "fn"
start_line: 813
end_line: 820
---

# wildcard

- Fragment ID: `rgbdns-frag-9e00c4c01f81`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 813-820
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9e00c4c01f81", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-9e00c4c01f81: fn wildcard", "sourcePath": "src/zone.rs", "startLine": 813, "endLine": 820}
```

## Excerpt

<span id="rgbdns-frag-9e00c4c01f81" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9e00c4c01f81: fn wildcard

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
