---
type: "code-fragment"
fragment_id: "rgbdns-frag-e62cd14dbf49"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "append_tsig_variables"
kind: "fn"
start_line: 1092
end_line: 1112
---

# append_tsig_variables

- Fragment ID: `rgbdns-frag-e62cd14dbf49`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 1092-1112
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e62cd14dbf49", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 1092, "endLine": 1112}
```

## Excerpt

<span id="rgbdns-frag-e62cd14dbf49" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e62cd14dbf49: fn append_tsig_variables

```rust
fn append_tsig_variables(
    output: &mut Vec<u8>,
    key: &Name,
    algorithm: &Name,
    time: u64,
    fudge: u16,
    error: u16,
    other: &[u8],
) -> Result<()> {
    encode_name_canonical(output, key)?;
    output.extend_from_slice(&CLASS_ANY.to_be_bytes());
    output.extend_from_slice(&0u32.to_be_bytes());
    encode_name_canonical(output, algorithm)?;
    output.extend_from_slice(&time.to_be_bytes()[2..]);
    output.extend_from_slice(&fudge.to_be_bytes());
    output.extend_from_slice(&error.to_be_bytes());
    output.extend_from_slice(&(other.len() as u16).to_be_bytes());
    output.extend_from_slice(other);
    Ok(())
}

```
