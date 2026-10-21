---
type: "code-fragment"
fragment_id: "rgbdns-frag-dcbd06740aff"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "split_tinydns"
kind: "fn"
start_line: 766
end_line: 790
---

# split_tinydns

- Fragment ID: `rgbdns-frag-dcbd06740aff`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 766-790
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-dcbd06740aff", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 766, "endLine": 790}
```

## Excerpt

<span id="rgbdns-frag-dcbd06740aff" class="rgbdns-fragment-target"></span>
### rgbdns-frag-dcbd06740aff: fn split_tinydns

```rust
fn split_tinydns(value: &str) -> Result<Vec<String>> {
    let mut fields = vec![String::new()];
    let bytes = value.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'\\' {
            let escape = bytes
                .get(index..index + 4)
                .ok_or(Error::Format("truncated ACME overlay escape"))?;
            fields
                .last_mut()
                .unwrap()
                .push_str(std::str::from_utf8(escape).map_err(|_| Error::Format("bad overlay"))?);
            index += 4;
        } else if bytes[index] == b':' {
            fields.push(String::new());
            index += 1;
        } else {
            fields.last_mut().unwrap().push(char::from(bytes[index]));
            index += 1;
        }
    }
    Ok(fields)
}

```
