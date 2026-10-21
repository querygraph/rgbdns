---
type: "code-fragment"
fragment_id: "rgbdns-frag-6374c8fa1b33"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "validate_overlay"
kind: "fn"
start_line: 615
end_line: 625
---

# validate_overlay

- Fragment ID: `rgbdns-frag-6374c8fa1b33`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 615-625
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-6374c8fa1b33", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 615, "endLine": 625}
```

## Excerpt

<span id="rgbdns-frag-6374c8fa1b33" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6374c8fa1b33: fn validate_overlay

```rust
fn validate_overlay(overlay: &Overlay, policies: &BTreeMap<Name, Policy>) -> Result<()> {
    validate_overlay_counts(overlay)?;
    if overlay
        .keys()
        .any(|owner| !policies.values().any(|policy| authorized(policy, owner)))
    {
        return Err(Error::Format("ACME overlay contains unauthorized owner"));
    }
    Ok(())
}

```
