---
type: "code-fragment"
fragment_id: "rgbdns-frag-8ea20fcad487"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "sync_parent"
kind: "fn"
start_line: 847
end_line: 856
---

# sync_parent

- Fragment ID: `rgbdns-frag-8ea20fcad487`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 847-856
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-8ea20fcad487", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 847, "endLine": 856}
```

## Excerpt

<span id="rgbdns-frag-8ea20fcad487" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8ea20fcad487: fn sync_parent

```rust
pub fn sync_parent(path: &Path) -> Result<()> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    File::open(parent)?.sync_all()?;
    Ok(())
}

#[cfg(test)]
```
