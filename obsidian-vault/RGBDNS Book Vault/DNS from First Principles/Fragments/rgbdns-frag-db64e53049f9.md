---
type: "code-fragment"
fragment_id: "rgbdns-frag-db64e53049f9"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "patched_srv_marker_and_glue"
kind: "fn"
start_line: 755
end_line: 774
---

# patched_srv_marker_and_glue

- Fragment ID: `rgbdns-frag-db64e53049f9`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 755-774
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-db64e53049f9", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-db64e53049f9: fn patched_srv_marker_and_glue", "sourcePath": "src/zone.rs", "startLine": 755, "endLine": 774}
```

## Excerpt

<span id="rgbdns-frag-db64e53049f9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-db64e53049f9: fn patched_srv_marker_and_glue

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
