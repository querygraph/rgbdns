---
type: "code-fragment"
fragment_id: "rgbdns-frag-887f4bda18f1"
source_path: "tests/drill_interop.rs"
code_note: "DNS from First Principles/Code/tests/drill_interop.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "ldns_drill_interoperates_over_udp_tcp_edns_and_unknown_types"
kind: "fn"
start_line: 85
end_line: 106
---

# ldns_drill_interoperates_over_udp_tcp_edns_and_unknown_types

- Fragment ID: `rgbdns-frag-887f4bda18f1`
- Source file: [[DNS from First Principles/Code/tests/drill_interop.rs.source|tests/drill_interop.rs]]
- Lines: 85-106
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-887f4bda18f1", "codeNote": "DNS from First Principles/Code/tests/drill_interop.rs.source", "heading": "rgbdns-frag-887f4bda18f1: fn ldns_drill_interoperates_over_udp_tcp_edns_and_unknown_types", "sourcePath": "tests/drill_interop.rs", "startLine": 85, "endLine": 106}
```

## Excerpt

<span id="rgbdns-frag-887f4bda18f1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-887f4bda18f1: fn ldns_drill_interoperates_over_udp_tcp_edns_and_unknown_types

```rust
fn ldns_drill_interoperates_over_udp_tcp_edns_and_unknown_types() {
    if !drill_available() {
        eprintln!("skipping: ldns drill is not installed");
        return;
    }
    let (_server, port) = start_server();

    let udp = drill(port, &["-u"], "WwW.ExAmPlE", "A");
    assert!(udp.contains("192.0.2.1"));
    assert!(udp.contains("rcode: NOERROR"));

    let tcp = drill(port, &["-t"], "www.example", "AAAA");
    assert!(tcp.contains("2001:db8::1"));

    let edns = drill(port, &["-D", "-b", "1232"], "txt.example", "TXT");
    assert!(edns.contains("\"hello\""));
    assert!(edns.contains("EDNS"));

    let unknown = drill(port, &[], "www.example", "TYPE65000");
    assert!(unknown.contains("rcode: NOERROR"));
    assert!(unknown.contains("ANSWER: 0"));
}
```
