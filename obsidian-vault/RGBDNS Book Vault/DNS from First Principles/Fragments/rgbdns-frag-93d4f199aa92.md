---
type: "code-fragment"
fragment_id: "rgbdns-frag-93d4f199aa92"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "atomic_write"
kind: "fn"
start_line: 677
end_line: 692
---

# atomic_write

- Fragment ID: `rgbdns-frag-93d4f199aa92`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 677-692
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-93d4f199aa92", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 677, "endLine": 692}
```

## Excerpt

<span id="rgbdns-frag-93d4f199aa92" class="rgbdns-fragment-target"></span>
### rgbdns-frag-93d4f199aa92: fn atomic_write

```rust
fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let temporary = path.with_extension(format!("new.{}", std::process::id()));
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temporary)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    fs::rename(&temporary, path)?;
    FileSync::sync_directory(
        path.parent()
            .ok_or(Error::Format("state path has no parent"))?,
    )?;
    Ok(())
}

```
