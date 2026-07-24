---
type: "code-fragment"
fragment_id: "rgbdns-frag-47c3cd5b1372"
source_path: "src/conf.rs"
code_note: "DNS from First Principles/Code/src/conf.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "configure_tinydns"
kind: "fn"
start_line: 143
end_line: 176
---

# configure_tinydns

- Fragment ID: `rgbdns-frag-47c3cd5b1372`
- Source file: [[DNS from First Principles/Code/src/conf.rs.source|src/conf.rs]]
- Lines: 143-176
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-47c3cd5b1372", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-47c3cd5b1372: fn configure_tinydns", "sourcePath": "src/conf.rs", "startLine": 143, "endLine": 176}
```

## Excerpt

<span id="rgbdns-frag-47c3cd5b1372" class="rgbdns-fragment-target"></span>
### rgbdns-frag-47c3cd5b1372: fn configure_tinydns

```rust
fn configure_tinydns(directory: &Path, root: &Path) -> Result<()> {
    write_file(&root.join("data"), b"", 0o644)?;
    write_file(
        &root.join("Makefile"),
        format!(
            "data.cdb: data\n\t{}\n",
            executable("tinydns-data")?.display()
        )
        .as_bytes(),
        0o644,
    )?;
    for (script, mode) in [
        ("add-ns", "ns"),
        ("add-childns", "childns"),
        ("add-host", "host"),
        ("add-alias", "alias"),
        ("add-host6", "host6"),
        ("add-alias6", "alias6"),
        ("add-mx", "mx"),
    ] {
        write_file(
            &root.join(script),
            format!(
                "#!/bin/sh\nexec {} data data.new add {mode} \"$@\"\n",
                shell_quote(&executable("tinydns-edit")?.to_string_lossy())
            )
            .as_bytes(),
            0o755,
        )?;
    }
    let _ = directory;
    Ok(())
}

```
