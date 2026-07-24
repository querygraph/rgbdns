---
type: "code-fragment"
fragment_id: "rgbdns-frag-71ec2b6bd4ef"
source_path: "src/conf.rs"
code_note: "DNS from First Principles/Code/src/conf.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "shell_quote"
kind: "fn"
start_line: 231
end_line: 234
---

# shell_quote

- Fragment ID: `rgbdns-frag-71ec2b6bd4ef`
- Source file: [[DNS from First Principles/Code/src/conf.rs.source|src/conf.rs]]
- Lines: 231-234
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-71ec2b6bd4ef", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-71ec2b6bd4ef: fn shell_quote", "sourcePath": "src/conf.rs", "startLine": 231, "endLine": 234}
```

## Excerpt

<span id="rgbdns-frag-71ec2b6bd4ef" class="rgbdns-fragment-target"></span>
### rgbdns-frag-71ec2b6bd4ef: fn shell_quote

```rust
fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

```
