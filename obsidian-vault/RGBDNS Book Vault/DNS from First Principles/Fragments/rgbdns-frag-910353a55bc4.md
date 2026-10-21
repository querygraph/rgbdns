---
type: "code-fragment"
fragment_id: "rgbdns-frag-910353a55bc4"
source_path: "src/bin/rgbdns-log-report.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbdns-log-report"
symbol: "aggregate"
kind: "fn"
start_line: 114
end_line: 133
---

# aggregate

- Fragment ID: `rgbdns-frag-910353a55bc4`
- Source file: [[DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source|src/bin/rgbdns-log-report.rs]]
- Lines: 114-133
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbdns-log-report|rgbdns-log-report]]

```rgbdns-fragment
{"id": "rgbdns-frag-910353a55bc4", "codeNote": "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source", "sourcePath": "src/bin/rgbdns-log-report.rs", "startLine": 114, "endLine": 133}
```

## Excerpt

<span id="rgbdns-frag-910353a55bc4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-910353a55bc4: fn aggregate

```rust
fn aggregate(reader: impl BufRead, zones: &[String]) -> Result<HashMap<String, Count>, String> {
    let mut counts: HashMap<String, Count> = zones
        .iter()
        .map(|zone| (zone.clone(), Count::default()))
        .collect();
    for line in reader.lines() {
        let line = line.map_err(|error| format!("cannot read query log: {error}"))?;
        let Some((client, name)) = parse_query(&line) else {
            continue;
        };
        let Some(zone) = containing_zone(&name, zones) else {
            continue;
        };
        let count = counts.get_mut(zone).expect("configured zone must exist");
        count.total += 1;
        count.clients.insert(client.to_ascii_lowercase());
    }
    Ok(counts)
}

```
