---
type: "code-fragment"
fragment_id: "rgbdns-frag-1997fc89b5d1"
source_path: "src/dnscache_config.rs"
code_note: "DNS from First Principles/Code/src/dnscache_config.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "private_temporary_path"
kind: "fn"
start_line: 167
end_line: 186
---

# private_temporary_path

- Fragment ID: `rgbdns-frag-1997fc89b5d1`
- Source file: [[DNS from First Principles/Code/src/dnscache_config.rs.source|src/dnscache_config.rs]]
- Lines: 167-186
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1997fc89b5d1", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-1997fc89b5d1: fn private_temporary_path", "sourcePath": "src/dnscache_config.rs", "startLine": 167, "endLine": 186}
```

## Excerpt

<span id="rgbdns-frag-1997fc89b5d1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1997fc89b5d1: fn private_temporary_path

```rust
fn private_temporary_path() -> Result<PathBuf> {
    for _ in 0..32 {
        let mut random = [0_u8; 16];
        fill(&mut random)
            .map_err(|_| Error::Io(std::io::Error::other("OS randomness unavailable")))?;
        let suffix = random
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        let path = env::temp_dir().join(format!("rgbdns-roots-{suffix}.zone"));
        if !path.exists() {
            return Ok(path);
        }
    }
    Err(Error::Io(std::io::Error::new(
        std::io::ErrorKind::AlreadyExists,
        "unable to allocate temporary root hints file",
    )))
}

```
