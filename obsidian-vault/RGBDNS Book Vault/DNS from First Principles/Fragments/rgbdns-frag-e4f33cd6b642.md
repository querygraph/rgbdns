---
type: "code-fragment"
fragment_id: "rgbdns-frag-e4f33cd6b642"
source_path: "src/tinydns_edit.rs"
code_note: "DNS from First Principles/Code/src/tinydns_edit.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "parse_flat_ipv6"
kind: "fn"
start_line: 168
end_line: 176
---

# parse_flat_ipv6

- Fragment ID: `rgbdns-frag-e4f33cd6b642`
- Source file: [[DNS from First Principles/Code/src/tinydns_edit.rs.source|src/tinydns_edit.rs]]
- Lines: 168-176
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e4f33cd6b642", "codeNote": "DNS from First Principles/Code/src/tinydns_edit.rs.source", "heading": "rgbdns-frag-e4f33cd6b642: fn parse_flat_ipv6", "sourcePath": "src/tinydns_edit.rs", "startLine": 168, "endLine": 176}
```

## Excerpt

<span id="rgbdns-frag-e4f33cd6b642" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e4f33cd6b642: fn parse_flat_ipv6

```rust
fn parse_flat_ipv6(value: &str) -> Result<Ipv6Addr> {
    if value.len() != 32 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(Error::InvalidRecord("bad flat IPv6 address".into()));
    }
    Ok(Ipv6Addr::from(u128::from_str_radix(value, 16).map_err(
        |_| Error::InvalidRecord("bad flat IPv6 address".into()),
    )?))
}

```
