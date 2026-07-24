---
type: "code-fragment"
fragment_id: "rgbdns-frag-8d3f2bd5f0c8"
source_path: "src/bin/dnsfilter.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsfilter.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsfilter"
symbol: "options"
kind: "fn"
start_line: 36
end_line: 52
---

# options

- Fragment ID: `rgbdns-frag-8d3f2bd5f0c8`
- Source file: [[DNS from First Principles/Code/src/bin/dnsfilter.rs.source|src/bin/dnsfilter.rs]]
- Lines: 36-52
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsfilter|dnsfilter]]

```rgbdns-fragment
{"id": "rgbdns-frag-8d3f2bd5f0c8", "codeNote": "DNS from First Principles/Code/src/bin/dnsfilter.rs.source", "heading": "rgbdns-frag-8d3f2bd5f0c8: fn options", "sourcePath": "src/bin/dnsfilter.rs", "startLine": 36, "endLine": 52}
```

## Excerpt

<span id="rgbdns-frag-8d3f2bd5f0c8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8d3f2bd5f0c8: fn options

```rust
fn options() -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let mut concurrency = 10;
    let mut line_limit = 1000;
    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        let value = arguments
            .next()
            .ok_or("usage: dnsfilter [ -c concurrency ] [ -l lines ]")?;
        match argument.as_str() {
            "-c" => concurrency = value.parse::<usize>()?.clamp(1, 1000),
            "-l" => line_limit = value.parse::<usize>()?.clamp(1, 1_000_000),
            _ => return Err("usage: dnsfilter [ -c concurrency ] [ -l lines ]".into()),
        }
    }
    Ok((concurrency, line_limit))
}

```
