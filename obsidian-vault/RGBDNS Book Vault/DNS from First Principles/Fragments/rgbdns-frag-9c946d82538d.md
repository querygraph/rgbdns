---
type: "code-fragment"
fragment_id: "rgbdns-frag-9c946d82538d"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "patched_srv_marker_and_glue"
kind: "fn"
start_line: 1093
end_line: 1112
---

# patched_srv_marker_and_glue

- Fragment ID: `rgbdns-frag-9c946d82538d`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 1093-1112
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9c946d82538d", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 1093, "endLine": 1112}
```

## Excerpt

<span id="rgbdns-frag-9c946d82538d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9c946d82538d: fn patched_srv_marker_and_glue

```rust
    fn patched_srv_marker_and_glue() {
        let z = Zone::parse("S_sip._tcp.example:192.0.2.7:sip:5060:10:20:300\n").unwrap();
        assert!(matches!(
            z.lookup(
                &"_sip._tcp.example".parse().unwrap(),
                RecordType::Srv
            ),
            Lookup::Answer(records)
                if matches!(
                    &records[0].data,
                    RData::Srv { priority: 10, weight: 20, port: 5060, target }
                        if target.to_string() == "sip.srv._sip._tcp.example."
                )
        ));
        assert!(matches!(
            z.lookup(&"sip.srv._sip._tcp.example".parse().unwrap(), RecordType::A),
            Lookup::Answer(_)
        ));
    }
    #[test]
```
