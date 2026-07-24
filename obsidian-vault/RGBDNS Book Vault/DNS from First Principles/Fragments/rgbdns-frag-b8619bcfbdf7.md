---
type: "code-fragment"
fragment_id: "rgbdns-frag-b8619bcfbdf7"
source_path: "src/client.rs"
code_note: "DNS from First Principles/Code/src/client.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "parses_bare_and_explicit_port_server_addresses"
kind: "fn"
start_line: 208
end_line: 219
---

# parses_bare_and_explicit_port_server_addresses

- Fragment ID: `rgbdns-frag-b8619bcfbdf7`
- Source file: [[DNS from First Principles/Code/src/client.rs.source|src/client.rs]]
- Lines: 208-219
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b8619bcfbdf7", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "heading": "rgbdns-frag-b8619bcfbdf7: fn parses_bare_and_explicit_port_server_addresses", "sourcePath": "src/client.rs", "startLine": 208, "endLine": 219}
```

## Excerpt

<span id="rgbdns-frag-b8619bcfbdf7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b8619bcfbdf7: fn parses_bare_and_explicit_port_server_addresses

```rust
    fn parses_bare_and_explicit_port_server_addresses() {
        assert_eq!(
            server_address("192.0.2.1").unwrap(),
            "192.0.2.1:53".parse().unwrap()
        );
        assert_eq!(
            server_address("127.0.0.1:5353").unwrap(),
            "127.0.0.1:5353".parse().unwrap()
        );
    }

    #[test]
```
