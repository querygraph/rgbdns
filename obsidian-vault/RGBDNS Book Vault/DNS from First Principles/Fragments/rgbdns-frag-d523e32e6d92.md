---
type: "code-fragment"
fragment_id: "rgbdns-frag-d523e32e6d92"
source_path: "tests/packet_properties.rs"
code_note: "DNS from First Principles/Code/tests/packet_properties.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "dns_name"
kind: "fn"
start_line: 5
end_line: 9
---

# dns_name

- Fragment ID: `rgbdns-frag-d523e32e6d92`
- Source file: [[DNS from First Principles/Code/tests/packet_properties.rs.source|tests/packet_properties.rs]]
- Lines: 5-9
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-d523e32e6d92", "codeNote": "DNS from First Principles/Code/tests/packet_properties.rs.source", "heading": "rgbdns-frag-d523e32e6d92: fn dns_name", "sourcePath": "tests/packet_properties.rs", "startLine": 5, "endLine": 9}
```

## Excerpt

<span id="rgbdns-frag-d523e32e6d92" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d523e32e6d92: fn dns_name

```rust
fn dns_name() -> impl Strategy<Value = rgbdns::Name> {
    prop::collection::vec("[a-z0-9]{1,20}", 1..=4)
        .prop_map(|labels| labels.join(".").parse().unwrap())
}

```
