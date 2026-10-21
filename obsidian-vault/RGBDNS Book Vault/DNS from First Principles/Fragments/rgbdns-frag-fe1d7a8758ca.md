---
type: "code-fragment"
fragment_id: "rgbdns-frag-fe1d7a8758ca"
source_path: "src/bin/rgbdns-log-report.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbdns-log-report"
symbol: "Count"
kind: "struct"
start_line: 11
end_line: 16
---

# Count

- Fragment ID: `rgbdns-frag-fe1d7a8758ca`
- Source file: [[DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source|src/bin/rgbdns-log-report.rs]]
- Lines: 11-16
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbdns-log-report|rgbdns-log-report]]

```rgbdns-fragment
{"id": "rgbdns-frag-fe1d7a8758ca", "codeNote": "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source", "sourcePath": "src/bin/rgbdns-log-report.rs", "startLine": 11, "endLine": 16}
```

## Excerpt

<span id="rgbdns-frag-fe1d7a8758ca" class="rgbdns-fragment-target"></span>
### rgbdns-frag-fe1d7a8758ca: struct Count

```rust
struct Count {
    total: u64,
    clients: HashSet<String>,
}

#[derive(Debug)]
```
