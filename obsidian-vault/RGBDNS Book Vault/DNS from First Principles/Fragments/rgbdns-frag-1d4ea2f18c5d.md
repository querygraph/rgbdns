---
type: "code-fragment"
fragment_id: "rgbdns-frag-1d4ea2f18c5d"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "rfc6891_edns_is_acknowledged_and_do_is_copied"
kind: "fn"
start_line: 125
end_line: 145
---

# rfc6891_edns_is_acknowledged_and_do_is_copied

- Fragment ID: `rgbdns-frag-1d4ea2f18c5d`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 125-145
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-1d4ea2f18c5d", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-1d4ea2f18c5d: fn rfc6891_edns_is_acknowledged_and_do_is_copied", "sourcePath": "tests/rfc_conformance.rs", "startLine": 125, "endLine": 145}
```

## Excerpt

<span id="rgbdns-frag-1d4ea2f18c5d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1d4ea2f18c5d: fn rfc6891_edns_is_acknowledged_and_do_is_copied

```rust
fn rfc6891_edns_is_acknowledged_and_do_is_copied() {
    for flags in [0, 0x8000, 0x7fff, 0xffff] {
        let mut request = query("www.example", RecordType::A);
        request.additionals.push(opt(1232, 0, flags, Vec::new()));
        let response = response(&request);
        let RData::Opt {
            udp_payload,
            version,
            flags: response_flags,
            ..
        } = response.additionals.last().unwrap().data
        else {
            panic!("missing OPT response");
        };
        assert_eq!(udp_payload, 1232);
        assert_eq!(version, 0);
        assert_eq!(response_flags, flags & 0x8000);
    }
}

#[test]
```
