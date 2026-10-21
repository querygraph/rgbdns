---
type: "code-fragment"
fragment_id: "rgbdns-frag-93aaa33a10a5"
source_path: "src/bin/rgbdns-log-report.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbdns-log-report"
symbol: "render"
kind: "fn"
start_line: 134
end_line: 162
---

# render

- Fragment ID: `rgbdns-frag-93aaa33a10a5`
- Source file: [[DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source|src/bin/rgbdns-log-report.rs]]
- Lines: 134-162
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbdns-log-report|rgbdns-log-report]]

```rgbdns-fragment
{"id": "rgbdns-frag-93aaa33a10a5", "codeNote": "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source", "sourcePath": "src/bin/rgbdns-log-report.rs", "startLine": 134, "endLine": 162}
```

## Excerpt

<span id="rgbdns-frag-93aaa33a10a5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-93aaa33a10a5: fn render

```rust
fn render(date: &str, counts: HashMap<String, Count>) -> String {
    let mut counts: Vec<_> = counts.into_iter().collect();
    counts.sort_by(|(left_zone, left), (right_zone, right)| {
        right
            .total
            .cmp(&left.total)
            .then_with(|| right.clients.len().cmp(&left.clients.len()))
            .then_with(|| left_zone.cmp(right_zone))
    });
    let total: u64 = counts.iter().map(|(_, count)| count.total).sum();
    let mut output = format!(
        "rgbdns daily query report for {date}\n\n{:<40} {:>12} {:>12}\n{:-<40} {:-<12} {:-<12}\n",
        "DOMAIN", "TOTAL", "UNIQUE", "", "", ""
    );
    for (zone, count) in counts {
        output.push_str(&format!(
            "{zone:<40} {:>12} {:>12}\n",
            count.total,
            count.clients.len()
        ));
    }
    output.push_str(&format!(
        "\nTotal accepted DNS queries: {total}\n\n\
         Unique counts are distinct client/resolver IP addresses per domain.\n\
         They are not unique people or HTTP pageviews.\n"
    ));
    output
}

```
