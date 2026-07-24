---
type: "code-fragment"
fragment_id: "rgbdns-frag-950f5158793d"
source_path: "src/conf.rs"
code_note: "DNS from First Principles/Code/src/conf.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "configure_dnscache"
kind: "fn"
start_line: 177
end_line: 191
---

# configure_dnscache

- Fragment ID: `rgbdns-frag-950f5158793d`
- Source file: [[DNS from First Principles/Code/src/conf.rs.source|src/conf.rs]]
- Lines: 177-191
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-950f5158793d", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-950f5158793d: fn configure_dnscache", "sourcePath": "src/conf.rs", "startLine": 177, "endLine": 191}
```

## Excerpt

<span id="rgbdns-frag-950f5158793d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-950f5158793d: fn configure_dnscache

```rust
fn configure_dnscache(directory: &Path, root: &Path) -> Result<()> {
    let hints = include_bytes!("../config/root.hints");
    fs::create_dir(root.join("servers"))?;
    write_file(&root.join("servers/@"), hints, 0o644)?;
    write_file(
        &directory.join("env/ROOTS"),
        format!("{}\n", root.join("servers/@").display()).as_bytes(),
        0o644,
    )?;
    let mut seed = [0; 128];
    getrandom::fill(&mut seed)
        .map_err(|_| Error::Io(std::io::Error::other("OS randomness unavailable")))?;
    write_file(&directory.join("seed"), &seed, 0o600)
}

```
