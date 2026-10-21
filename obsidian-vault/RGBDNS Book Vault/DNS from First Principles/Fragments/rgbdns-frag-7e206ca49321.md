---
type: "code-fragment"
fragment_id: "rgbdns-frag-7e206ca49321"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "decode_txt"
kind: "fn"
start_line: 574
end_line: 593
---

# decode_txt

- Fragment ID: `rgbdns-frag-7e206ca49321`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 574-593
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7e206ca49321", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 574, "endLine": 593}
```

## Excerpt

<span id="rgbdns-frag-7e206ca49321" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7e206ca49321: fn decode_txt

```rust
fn decode_txt(rdata: &[u8]) -> Result<Vec<u8>> {
    let mut input = rdata;
    let mut value = Vec::new();
    while !input.is_empty() {
        let length = usize::from(input[0]);
        input = input
            .get(1..)
            .ok_or(Error::Format("truncated TXT update"))?;
        let chunk = input
            .get(..length)
            .ok_or(Error::Format("truncated TXT update"))?;
        value.extend_from_slice(chunk);
        input = &input[length..];
    }
    if value.is_empty() || value.len() > 255 {
        return Err(Error::Format("ACME TXT value length is invalid"));
    }
    Ok(value)
}

```
