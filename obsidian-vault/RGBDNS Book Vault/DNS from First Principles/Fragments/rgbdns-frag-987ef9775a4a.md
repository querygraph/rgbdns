---
type: "code-fragment"
fragment_id: "rgbdns-frag-987ef9775a4a"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "escape_tinydns"
kind: "fn"
start_line: 754
end_line: 765
---

# escape_tinydns

- Fragment ID: `rgbdns-frag-987ef9775a4a`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 754-765
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-987ef9775a4a", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 754, "endLine": 765}
```

## Excerpt

<span id="rgbdns-frag-987ef9775a4a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-987ef9775a4a: fn escape_tinydns

```rust
fn escape_tinydns(value: &[u8]) -> String {
    let mut output = String::new();
    for byte in value {
        if (b'!'..=b'~').contains(byte) && !matches!(byte, b':' | b'\\') {
            output.push(char::from(*byte));
        } else {
            output.push_str(&format!("\\{:03o}", byte));
        }
    }
    output
}

```
