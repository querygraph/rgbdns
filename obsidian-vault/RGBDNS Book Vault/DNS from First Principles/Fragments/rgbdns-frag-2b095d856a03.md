---
type: "code-fragment"
fragment_id: "rgbdns-frag-2b095d856a03"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "encode_name_canonical"
kind: "fn"
start_line: 1122
end_line: 1130
---

# encode_name_canonical

- Fragment ID: `rgbdns-frag-2b095d856a03`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 1122-1130
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-2b095d856a03", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 1122, "endLine": 1130}
```

## Excerpt

<span id="rgbdns-frag-2b095d856a03" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2b095d856a03: fn encode_name_canonical

```rust
fn encode_name_canonical(output: &mut Vec<u8>, name: &Name) -> Result<()> {
    for label in name.labels() {
        output.push(u8::try_from(label.len()).map_err(|_| Error::Format("long TSIG label"))?);
        output.extend(label.iter().map(u8::to_ascii_lowercase));
    }
    output.push(0);
    Ok(())
}

```
