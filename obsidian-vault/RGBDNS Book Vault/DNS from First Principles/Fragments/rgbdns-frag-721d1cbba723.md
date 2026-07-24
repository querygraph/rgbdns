---
type: "code-fragment"
fragment_id: "rgbdns-frag-721d1cbba723"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "wildcard"
kind: "fn"
start_line: 743
end_line: 750
---

# wildcard

- Fragment ID: `rgbdns-frag-721d1cbba723`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 743-750
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-721d1cbba723", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-721d1cbba723: fn wildcard", "sourcePath": "src/zone.rs", "startLine": 743, "endLine": 750}
```

## Excerpt

<span id="rgbdns-frag-721d1cbba723" class="rgbdns-fragment-target"></span>
### rgbdns-frag-721d1cbba723: fn wildcard

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
