---
type: "code-fragment"
fragment_id: "rgbdns-frag-a6f2c2224be1"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "atomic_copy"
kind: "fn"
start_line: 697
end_line: 710
---

# atomic_copy

- Fragment ID: `rgbdns-frag-a6f2c2224be1`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 697-710
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a6f2c2224be1", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 697, "endLine": 710}
```

## Excerpt

<span id="rgbdns-frag-a6f2c2224be1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a6f2c2224be1: fn atomic_copy

```rust
fn atomic_copy(input: &Path, output: &Path) -> Result<()> {
    let temporary = sibling_temporary(output);
    if let Err(error) = (|| -> Result<()> {
        fs::copy(input, &temporary)?;
        File::open(&temporary)?.sync_all()?;
        fs::rename(&temporary, output)?;
        sync_parent(output)
    })() {
        let _ = fs::remove_file(&temporary);
        return Err(error);
    }
    Ok(())
}

```
