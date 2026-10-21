---
type: "code-fragment"
fragment_id: "rgbdns-frag-087aaec14a3c"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "encode_txt"
kind: "fn"
start_line: 594
end_line: 603
---

# encode_txt

- Fragment ID: `rgbdns-frag-087aaec14a3c`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 594-603
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-087aaec14a3c", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 594, "endLine": 603}
```

## Excerpt

<span id="rgbdns-frag-087aaec14a3c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-087aaec14a3c: fn encode_txt

```rust
fn encode_txt(value: &[u8]) -> Result<Vec<u8>> {
    if value.is_empty() || value.len() > 255 {
        return Err(Error::Format("ACME TXT value length is invalid"));
    }
    let mut output = Vec::with_capacity(value.len() + 1);
    output.push(value.len() as u8);
    output.extend_from_slice(value);
    Ok(output)
}

```
