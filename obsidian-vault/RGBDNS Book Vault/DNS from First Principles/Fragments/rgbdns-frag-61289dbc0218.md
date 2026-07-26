---
type: "code-fragment"
fragment_id: "rgbdns-frag-61289dbc0218"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "constructs_ipv4_and_ipv6_socket_addresses"
kind: "fn"
start_line: 69
end_line: 81
---

# constructs_ipv4_and_ipv6_socket_addresses

- Fragment ID: `rgbdns-frag-61289dbc0218`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 69-81
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-61289dbc0218", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-61289dbc0218: fn constructs_ipv4_and_ipv6_socket_addresses", "sourcePath": "src/lib.rs", "startLine": 69, "endLine": 81}
```

## Excerpt

<span id="rgbdns-frag-61289dbc0218" class="rgbdns-fragment-target"></span>
### rgbdns-frag-61289dbc0218: fn constructs_ipv4_and_ipv6_socket_addresses

```rust
    fn constructs_ipv4_and_ipv6_socket_addresses() {
        assert_eq!(
            socket_address("192.0.2.1", "5353").unwrap().to_string(),
            "192.0.2.1:5353"
        );
        assert_eq!(
            socket_address("2001:db8::1", "53").unwrap().to_string(),
            "[2001:db8::1]:53"
        );
        assert!(socket_address("bad", "53").is_err());
        assert!(socket_address("127.0.0.1", "65536").is_err());
    }
}
```
