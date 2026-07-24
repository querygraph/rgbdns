---
type: "code-fragment"
fragment_id: "rgbdns-frag-1d5ad9c1babc"
source_path: "src/bin/dnsfilter.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsfilter.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsfilter"
symbol: "run"
kind: "fn"
start_line: 20
end_line: 35
---

# run

- Fragment ID: `rgbdns-frag-1d5ad9c1babc`
- Source file: [[DNS from First Principles/Code/src/bin/dnsfilter.rs.source|src/bin/dnsfilter.rs]]
- Lines: 20-35
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsfilter|dnsfilter]]

```rgbdns-fragment
{"id": "rgbdns-frag-1d5ad9c1babc", "codeNote": "DNS from First Principles/Code/src/bin/dnsfilter.rs.source", "heading": "rgbdns-frag-1d5ad9c1babc: fn run", "sourcePath": "src/bin/dnsfilter.rs", "startLine": 20, "endLine": 35}
```

## Excerpt

<span id="rgbdns-frag-1d5ad9c1babc" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1d5ad9c1babc: fn run

```rust
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (concurrency, line_limit) = options()?;
    let stdin = std::io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        lines.push(line?);
        if lines.len() == line_limit {
            process(std::mem::take(&mut lines), concurrency)?;
        }
    }
    if !lines.is_empty() {
        process(lines, concurrency)?;
    }
    Ok(())
}

```
