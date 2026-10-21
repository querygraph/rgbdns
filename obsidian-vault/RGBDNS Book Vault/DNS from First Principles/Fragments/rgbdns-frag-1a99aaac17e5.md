---
type: "code-fragment"
fragment_id: "rgbdns-frag-1a99aaac17e5"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "unescape_tinydns"
kind: "fn"
start_line: 791
end_line: 812
---

# unescape_tinydns

- Fragment ID: `rgbdns-frag-1a99aaac17e5`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 791-812
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1a99aaac17e5", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 791, "endLine": 812}
```

## Excerpt

<span id="rgbdns-frag-1a99aaac17e5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1a99aaac17e5: fn unescape_tinydns

```rust
fn unescape_tinydns(value: &str) -> Result<Vec<u8>> {
    let bytes = value.as_bytes();
    let mut output = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] != b'\\' {
            output.push(bytes[index]);
            index += 1;
            continue;
        }
        let digits = bytes
            .get(index + 1..index + 4)
            .ok_or(Error::Format("truncated ACME overlay escape"))?;
        if !digits.iter().all(|byte| (b'0'..=b'7').contains(byte)) {
            return Err(Error::Format("invalid ACME overlay escape"));
        }
        output.push((digits[0] - b'0') * 64 + (digits[1] - b'0') * 8 + digits[2] - b'0');
        index += 4;
    }
    Ok(output)
}

```
