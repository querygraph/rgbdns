---
type: "code-fragment"
fragment_id: "rgbdns-frag-a545b39517cc"
source_path: "src/dnscache_config.rs"
code_note: "DNS from First Principles/Code/src/dnscache_config.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "write_private"
kind: "fn"
start_line: 187
end_line: 201
---

# write_private

- Fragment ID: `rgbdns-frag-a545b39517cc`
- Source file: [[DNS from First Principles/Code/src/dnscache_config.rs.source|src/dnscache_config.rs]]
- Lines: 187-201
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a545b39517cc", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-a545b39517cc: fn write_private", "sourcePath": "src/dnscache_config.rs", "startLine": 187, "endLine": 201}
```

## Excerpt

<span id="rgbdns-frag-a545b39517cc" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a545b39517cc: fn write_private

```rust
fn write_private(path: &Path, contents: &[u8]) -> Result<()> {
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path)?;
    file.write_all(contents)?;
    file.sync_all()?;
    Ok(())
}

#[cfg(test)]
```
