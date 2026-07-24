---
type: "code-fragment"
fragment_id: "rgbdns-frag-631de8a93e74"
source_path: "src/bin/multilog.rs"
code_note: "DNS from First Principles/Code/src/bin/multilog.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "multilog"
symbol: "run"
kind: "fn"
start_line: 11
end_line: 16
---

# run

- Fragment ID: `rgbdns-frag-631de8a93e74`
- Source file: [[DNS from First Principles/Code/src/bin/multilog.rs.source|src/bin/multilog.rs]]
- Lines: 11-16
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/multilog|multilog]]

```rgbdns-fragment
{"id": "rgbdns-frag-631de8a93e74", "codeNote": "DNS from First Principles/Code/src/bin/multilog.rs.source", "heading": "rgbdns-frag-631de8a93e74: fn run", "sourcePath": "src/bin/multilog.rs", "startLine": 11, "endLine": 16}
```

## Excerpt

<span id="rgbdns-frag-631de8a93e74" class="rgbdns-fragment-target"></span>
### rgbdns-frag-631de8a93e74: fn run

```rust
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let config = Config::parse(&arguments)?;
    multilog::run(&config, BufReader::new(std::io::stdin()))?;
    Ok(())
}
```
