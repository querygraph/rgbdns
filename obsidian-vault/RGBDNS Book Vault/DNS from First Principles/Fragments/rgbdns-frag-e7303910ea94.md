---
type: "code-fragment"
fragment_id: "rgbdns-frag-e7303910ea94"
source_path: "src/bin/rgbdns-log-report.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbdns-log-report"
symbol: "aggregates_subdomains_by_longest_zone_and_sorts_by_total"
kind: "fn"
start_line: 193
end_line: 214
---

# aggregates_subdomains_by_longest_zone_and_sorts_by_total

- Fragment ID: `rgbdns-frag-e7303910ea94`
- Source file: [[DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source|src/bin/rgbdns-log-report.rs]]
- Lines: 193-214
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbdns-log-report|rgbdns-log-report]]

```rgbdns-fragment
{"id": "rgbdns-frag-e7303910ea94", "codeNote": "DNS from First Principles/Code/src/bin/rgbdns-log-report.rs.source", "sourcePath": "src/bin/rgbdns-log-report.rs", "startLine": 193, "endLine": 214}
```

## Excerpt

<span id="rgbdns-frag-e7303910ea94" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e7303910ea94: fn aggregates_subdomains_by_longest_zone_and_sorts_by_total

```rust
    fn aggregates_subdomains_by_longest_zone_and_sorts_by_total() {
        let zones = read_zones(Cursor::new("example\nexample.com\nquiet.test\n")).unwrap();
        let logs = concat!(
            "7f000001:e214:0018 + 0001 example.com\n",
            "7f000001:e215:0019 + 001c www.example.com\n",
            "08080808:0035:0020 + 0001 api.example.com.\n",
            "7f000001:e216:0021 + 0001 example\n",
            "7f000001:e217:0022 - 0001 refused.example.com\n",
            "starting tinydns\n",
        );
        let report = render("2026-08-04", aggregate(Cursor::new(logs), &zones).unwrap());
        let example_com = report.find("example.com").unwrap();
        let example = report.find("example                             ").unwrap();
        assert!(example_com < example);
        assert!(report[example_com..].starts_with("example.com"));
        assert!(report[example_com..].contains("           3            2"));
        assert!(report[example..].contains("           1            1"));
        assert!(report.contains("quiet.test"));
        assert!(report.contains("Total accepted DNS queries: 4"));
    }

    #[test]
```
