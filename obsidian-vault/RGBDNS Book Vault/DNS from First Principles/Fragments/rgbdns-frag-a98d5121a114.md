---
type: "code-fragment"
fragment_id: "rgbdns-frag-a98d5121a114"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "sibling_temporary"
kind: "fn"
start_line: 839
end_line: 846
---

# sibling_temporary

- Fragment ID: `rgbdns-frag-a98d5121a114`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 839-846
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a98d5121a114", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 839, "endLine": 846}
```

## Excerpt

<span id="rgbdns-frag-a98d5121a114" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a98d5121a114: fn sibling_temporary

```rust
fn sibling_temporary(output: &Path) -> PathBuf {
    let mut name = output
        .file_name()
        .map_or_else(|| "data.signed".into(), |name| name.to_os_string());
    name.push(".tmp");
    output.with_file_name(name)
}

```
