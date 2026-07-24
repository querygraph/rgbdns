---
type: "code-fragment"
fragment_id: "rgbdns-frag-b0dca80a842d"
source_path: "src/conf.rs"
code_note: "DNS from First Principles/Code/src/conf.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "write_file"
kind: "fn"
start_line: 235
end_line: 247
---

# write_file

- Fragment ID: `rgbdns-frag-b0dca80a842d`
- Source file: [[DNS from First Principles/Code/src/conf.rs.source|src/conf.rs]]
- Lines: 235-247
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b0dca80a842d", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-b0dca80a842d: fn write_file", "sourcePath": "src/conf.rs", "startLine": 235, "endLine": 247}
```

## Excerpt

<span id="rgbdns-frag-b0dca80a842d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b0dca80a842d: fn write_file

```rust
fn write_file(path: &Path, contents: &[u8], mode: u32) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents)?;
    file.sync_all()?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(mode))?;
    }
    Ok(())
}

#[cfg(test)]
```
