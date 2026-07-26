---
type: "code-fragment"
fragment_id: "rgbdns-frag-d484f0dd4859"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "common_markers"
kind: "fn"
start_line: 798
end_line: 812
---

# common_markers

- Fragment ID: `rgbdns-frag-d484f0dd4859`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 798-812
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d484f0dd4859", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-d484f0dd4859: fn common_markers", "sourcePath": "src/zone.rs", "startLine": 798, "endLine": 812}
```

## Excerpt

<span id="rgbdns-frag-d484f0dd4859" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d484f0dd4859: fn common_markers

```rust
    fn common_markers() {
        let z=Zone::parse(".example:192.0.2.53:ns.example\n=www.example:192.0.2.1:60\n'example:hello\\072world\n6v6.example:20010db8000000000000000000000001\n").unwrap();
        assert!(
            matches!(z.lookup(&"www.example".parse().unwrap(),RecordType::A),Lookup::Answer(x) if x[0].ttl==60)
        );
        assert!(matches!(
            z.lookup(&"missing.example".parse().unwrap(), RecordType::A),
            Lookup::NxDomain(Some(_))
        ));
        assert!(matches!(
            z.lookup(&"v6.example".parse().unwrap(), RecordType::Aaaa),
            Lookup::Answer(_)
        ))
    }
    #[test]
```
