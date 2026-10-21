---
type: "code-fragment"
fragment_id: "rgbdns-frag-c4be9dd3092a"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "validate_overlay_counts"
kind: "fn"
start_line: 604
end_line: 614
---

# validate_overlay_counts

- Fragment ID: `rgbdns-frag-c4be9dd3092a`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 604-614
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c4be9dd3092a", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 604, "endLine": 614}
```

## Excerpt

<span id="rgbdns-frag-c4be9dd3092a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c4be9dd3092a: fn validate_overlay_counts

```rust
fn validate_overlay_counts(overlay: &Overlay) -> Result<()> {
    if overlay.values().map(BTreeSet::len).sum::<usize>() > MAX_VALUES
        || overlay
            .values()
            .any(|values| values.len() > MAX_OWNER_VALUES)
    {
        return Err(Error::Format("ACME state limit exceeded"));
    }
    Ok(())
}

```
