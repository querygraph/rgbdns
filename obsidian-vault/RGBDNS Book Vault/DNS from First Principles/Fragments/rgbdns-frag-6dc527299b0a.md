---
type: "code-fragment"
fragment_id: "rgbdns-frag-6dc527299b0a"
source_path: "src/bin/rgbdns-log-report.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbdns-log-report"
symbol: "read_zones"
kind: "fn"
start_line: 56
end_line: 75
---

# read_zones

- Fragment ID: `rgbdns-frag-6dc527299b0a`
- Source file: [[DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source|src/bin/rgbdns-log-report.rs]]
- Lines: 56-75
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbdns-log-report|rgbdns-log-report]]

```rgbdns-fragment
{"id": "rgbdns-frag-6dc527299b0a", "codeNote": "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source", "sourcePath": "src/bin/rgbdns-log-report.rs", "startLine": 56, "endLine": 75}
```

## Excerpt

<span id="rgbdns-frag-6dc527299b0a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6dc527299b0a: fn read_zones

```rust
fn read_zones(reader: impl BufRead) -> Result<Vec<String>, String> {
    let mut zones = HashSet::new();
    for (number, line) in reader.lines().enumerate() {
        let line = line.map_err(|error| format!("cannot read zones: {error}"))?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let zone = normalize_name(line)
            .ok_or_else(|| format!("invalid zone on line {}: {line}", number + 1))?;
        zones.insert(zone);
    }
    let mut zones: Vec<_> = zones.into_iter().collect();
    zones.sort_by_key(|zone| Reverse(zone.len()));
    if zones.is_empty() {
        return Err("zone list is empty".to_owned());
    }
    Ok(zones)
}

```
