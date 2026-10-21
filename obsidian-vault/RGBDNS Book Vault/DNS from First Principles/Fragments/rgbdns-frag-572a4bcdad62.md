---
type: "code-fragment"
fragment_id: "rgbdns-frag-572a4bcdad62"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "encode_name"
kind: "fn"
start_line: 1113
end_line: 1121
---

# encode_name

- Fragment ID: `rgbdns-frag-572a4bcdad62`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 1113-1121
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-572a4bcdad62", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 1113, "endLine": 1121}
```

## Excerpt

<span id="rgbdns-frag-572a4bcdad62" class="rgbdns-fragment-target"></span>
### rgbdns-frag-572a4bcdad62: fn encode_name

```rust
fn encode_name(output: &mut Vec<u8>, name: &Name) -> Result<()> {
    for label in name.labels() {
        output.push(u8::try_from(label.len()).map_err(|_| Error::Format("long TSIG label"))?);
        output.extend_from_slice(label);
    }
    output.push(0);
    Ok(())
}

```
