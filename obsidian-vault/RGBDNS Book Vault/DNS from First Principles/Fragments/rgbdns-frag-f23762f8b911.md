---
type: "code-fragment"
fragment_id: "rgbdns-frag-f23762f8b911"
source_path: "src/conf.rs"
code_note: "DNS from First Principles/Code/src/conf.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "make_log"
kind: "fn"
start_line: 192
end_line: 207
---

# make_log

- Fragment ID: `rgbdns-frag-f23762f8b911`
- Source file: [[DNS from First Principles/Code/src/conf.rs.source|src/conf.rs]]
- Lines: 192-207
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f23762f8b911", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-f23762f8b911: fn make_log", "sourcePath": "src/conf.rs", "startLine": 192, "endLine": 207}
```

## Excerpt

<span id="rgbdns-frag-f23762f8b911" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f23762f8b911: fn make_log

```rust
fn make_log(directory: &Path, user: &str) -> Result<()> {
    fs::create_dir(directory.join("log"))?;
    fs::create_dir(directory.join("log/main"))?;
    write_file(
        &directory.join("log/run"),
        format!(
            "#!/bin/sh\nexec {} {} {} t ./main\n",
            shell_quote(&executable("setuidgid")?.to_string_lossy()),
            shell_quote(user),
            shell_quote(&executable("multilog")?.to_string_lossy()),
        )
        .as_bytes(),
        0o755,
    )
}

```
