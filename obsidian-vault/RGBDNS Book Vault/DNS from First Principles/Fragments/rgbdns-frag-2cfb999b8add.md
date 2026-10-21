---
type: "code-fragment"
fragment_id: "rgbdns-frag-2cfb999b8add"
source_path: "src/bin/rgbdns-log-report.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbdns-log-report"
symbol: "run"
kind: "fn"
start_line: 163
end_line: 172
---

# run

- Fragment ID: `rgbdns-frag-2cfb999b8add`
- Source file: [[DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source|src/bin/rgbdns-log-report.rs]]
- Lines: 163-172
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbdns-log-report|rgbdns-log-report]]

```rgbdns-fragment
{"id": "rgbdns-frag-2cfb999b8add", "codeNote": "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source", "sourcePath": "src/bin/rgbdns-log-report.rs", "startLine": 163, "endLine": 172}
```

## Excerpt

<span id="rgbdns-frag-2cfb999b8add" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2cfb999b8add: fn run

```rust
fn run() -> Result<(), String> {
    let options = options(env::args().skip(1))?;
    let zones_file = File::open(&options.zones)
        .map_err(|error| format!("cannot open {}: {error}", options.zones.display()))?;
    let zones = read_zones(BufReader::new(zones_file))?;
    let counts = aggregate(io::stdin().lock(), &zones)?;
    print!("{}", render(&options.date, counts));
    Ok(())
}

```
