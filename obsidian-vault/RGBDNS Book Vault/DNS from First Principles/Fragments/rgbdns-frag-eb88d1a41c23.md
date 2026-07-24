---
type: "code-fragment"
fragment_id: "rgbdns-frag-eb88d1a41c23"
source_path: "src/tinydns_edit.rs"
code_note: "DNS from First Principles/Code/src/tinydns_edit.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "display_address"
kind: "fn"
start_line: 161
end_line: 167
---

# display_address

- Fragment ID: `rgbdns-frag-eb88d1a41c23`
- Source file: [[DNS from First Principles/Code/src/tinydns_edit.rs.source|src/tinydns_edit.rs]]
- Lines: 161-167
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-eb88d1a41c23", "codeNote": "DNS from First Principles/Code/src/tinydns_edit.rs.source", "heading": "rgbdns-frag-eb88d1a41c23: fn display_address", "sourcePath": "src/tinydns_edit.rs", "startLine": 161, "endLine": 167}
```

## Excerpt

<span id="rgbdns-frag-eb88d1a41c23" class="rgbdns-fragment-target"></span>
### rgbdns-frag-eb88d1a41c23: fn display_address

```rust
fn display_address(address: Address) -> String {
    match address {
        Address::V4(address) => address.to_string(),
        Address::V6(address) => format!("{:032x}", u128::from(address)),
    }
}

```
