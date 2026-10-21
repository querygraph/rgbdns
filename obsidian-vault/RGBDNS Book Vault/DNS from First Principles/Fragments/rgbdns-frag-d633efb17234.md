---
type: "code-fragment"
fragment_id: "rgbdns-frag-d633efb17234"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "bare_response"
kind: "fn"
start_line: 1082
end_line: 1091
---

# bare_response

- Fragment ID: `rgbdns-frag-d633efb17234`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 1082-1091
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d633efb17234", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 1082, "endLine": 1091}
```

## Excerpt

<span id="rgbdns-frag-d633efb17234" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d633efb17234: fn bare_response

```rust
fn bare_response(request: &[u8], rcode: u16) -> Result<Vec<u8>> {
    if request.len() < 4 {
        return Err(Error::Format("short UPDATE"));
    }
    let mut response = vec![request[0], request[1]];
    response.extend_from_slice(&(0x8000 | OPCODE_UPDATE | (rcode & 0x0f)).to_be_bytes());
    response.extend_from_slice(&[0; 8]);
    Ok(response)
}

```
