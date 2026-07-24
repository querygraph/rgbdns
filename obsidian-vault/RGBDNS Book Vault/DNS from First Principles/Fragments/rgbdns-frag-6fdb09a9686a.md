---
type: "code-fragment"
fragment_id: "rgbdns-frag-6fdb09a9686a"
source_path: "src/tinydns_edit.rs"
code_note: "DNS from First Principles/Code/src/tinydns_edit.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "split_fields"
kind: "fn"
start_line: 209
end_line: 223
---

# split_fields

- Fragment ID: `rgbdns-frag-6fdb09a9686a`
- Source file: [[DNS from First Principles/Code/src/tinydns_edit.rs.source|src/tinydns_edit.rs]]
- Lines: 209-223
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-6fdb09a9686a", "codeNote": "DNS from First Principles/Code/src/tinydns_edit.rs.source", "heading": "rgbdns-frag-6fdb09a9686a: fn split_fields", "sourcePath": "src/tinydns_edit.rs", "startLine": 209, "endLine": 223}
```

## Excerpt

<span id="rgbdns-frag-6fdb09a9686a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6fdb09a9686a: fn split_fields

```rust
fn split_fields(value: &str) -> Vec<String> {
    let mut fields = vec![String::new()];
    let mut escaped = false;
    for character in value.chars() {
        if character == ':' && !escaped {
            fields.push(String::new());
        } else {
            fields.last_mut().unwrap().push(character);
        }
        escaped = character == '\\' && !escaped;
    }
    fields
}

#[cfg(test)]
```
