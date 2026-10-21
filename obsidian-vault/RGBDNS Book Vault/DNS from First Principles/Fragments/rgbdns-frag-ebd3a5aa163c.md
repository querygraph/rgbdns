---
type: "code-fragment"
fragment_id: "rgbdns-frag-ebd3a5aa163c"
source_path: "src/bin/rgbdns-log-report.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbdns-log-report"
symbol: "options"
kind: "fn"
start_line: 26
end_line: 45
---

# options

- Fragment ID: `rgbdns-frag-ebd3a5aa163c`
- Source file: [[DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source|src/bin/rgbdns-log-report.rs]]
- Lines: 26-45
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbdns-log-report|rgbdns-log-report]]

```rgbdns-fragment
{"id": "rgbdns-frag-ebd3a5aa163c", "codeNote": "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source", "sourcePath": "src/bin/rgbdns-log-report.rs", "startLine": 26, "endLine": 45}
```

## Excerpt

<span id="rgbdns-frag-ebd3a5aa163c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ebd3a5aa163c: fn options

```rust
fn options<I>(mut args: I) -> Result<Options, String>
where
    I: Iterator<Item = String>,
{
    let mut zones = None;
    let mut date = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--zones" => zones = args.next().map(PathBuf::from),
            "--date" => date = args.next(),
            "-h" | "--help" => return Err(usage().to_owned()),
            _ => return Err(format!("unknown argument: {arg}\n{}", usage())),
        }
    }
    Ok(Options {
        zones: zones.ok_or_else(|| format!("--zones is required\n{}", usage()))?,
        date: date.ok_or_else(|| format!("--date is required\n{}", usage()))?,
    })
}

```
