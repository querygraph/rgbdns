---
type: "code-fragment"
fragment_id: "rgbdns-frag-bc8df6e64c93"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "materialize_state"
kind: "fn"
start_line: 190
end_line: 212
---

# materialize_state

- Fragment ID: `rgbdns-frag-bc8df6e64c93`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 190-212
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-bc8df6e64c93", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 190, "endLine": 212}
```

## Excerpt

<span id="rgbdns-frag-bc8df6e64c93" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bc8df6e64c93: fn materialize_state

```rust
pub fn materialize_state(
    input: impl AsRef<Path>,
    config: impl AsRef<Path>,
    state_dir: impl AsRef<Path>,
    output: impl AsRef<Path>,
) -> Result<()> {
    let base = Zone::from_file(input)?;
    let policies = parse_config(&fs::read_to_string(config)?, &base)?;
    let state_dir = state_dir.as_ref();
    let overlay = read_overlay(&state_dir.join("acme-overlay.data"))?;
    let serials = read_serials(&state_dir.join("acme-serials"))?;
    validate_overlay(&overlay, &policies)?;
    let candidate = base.with_acme_overlay(&flatten(&overlay, &policies)?, &serials)?;
    let output = output.as_ref();
    let temporary = output.with_extension(format!("tmp.{}", std::process::id()));
    crate::axfr::write_zone(&candidate, output, &temporary)?;
    FileSync::sync_directory(
        output
            .parent()
            .ok_or(Error::Format("ACME materialized output has no parent"))?,
    )
}

```
