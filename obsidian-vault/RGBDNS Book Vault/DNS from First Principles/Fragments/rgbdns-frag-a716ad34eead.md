---
type: "code-fragment"
fragment_id: "rgbdns-frag-a716ad34eead"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "escape"
kind: "fn"
start_line: 357
end_line: 369
---

# escape

- Fragment ID: `rgbdns-frag-a716ad34eead`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 357-369
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a716ad34eead", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-a716ad34eead: fn escape", "sourcePath": "src/axfr.rs", "startLine": 357, "endLine": 369}
```

## Excerpt

<span id="rgbdns-frag-a716ad34eead" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a716ad34eead: fn escape

```rust
fn escape(bytes: &[u8]) -> String {
    let mut output = String::new();
    for byte in bytes {
        if (b'!'..=b'~').contains(byte) && *byte != b':' && *byte != b'\\' {
            output.push(char::from(*byte));
        } else {
            output.push_str(&format!("\\{byte:03o}"));
        }
    }
    output
}

#[cfg(test)]
```
