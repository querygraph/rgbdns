---
type: "code-fragment"
fragment_id: "rgbdns-frag-75f2307fa91a"
source_path: "src/bin/dnsfilter.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsfilter.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsfilter"
symbol: "filter_line"
kind: "fn"
start_line: 86
end_line: 119
---

# filter_line

- Fragment ID: `rgbdns-frag-75f2307fa91a`
- Source file: [[DNS from First Principles/Code/src/bin/dnsfilter.rs.source|src/bin/dnsfilter.rs]]
- Lines: 86-119
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsfilter|dnsfilter]]

```rgbdns-fragment
{"id": "rgbdns-frag-75f2307fa91a", "codeNote": "DNS from First Principles/Code/src/bin/dnsfilter.rs.source", "heading": "rgbdns-frag-75f2307fa91a: fn filter_line", "sourcePath": "src/bin/dnsfilter.rs", "startLine": 86, "endLine": 119}
```

## Excerpt

<span id="rgbdns-frag-75f2307fa91a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-75f2307fa91a: fn filter_line

```rust
fn filter_line(line: &str) -> String {
    let split = line.find([' ', '\t']).unwrap_or(line.len());
    let (left, right) = line.split_at(split);
    let Ok(address) = left.parse::<Ipv4Addr>() else {
        return line.to_owned();
    };
    let octets = address.octets();
    let reverse = format!(
        "{}.{}.{}.{}.in-addr.arpa",
        octets[3], octets[2], octets[1], octets[0]
    );
    match reverse
        .parse()
        .and_then(|name| client::recursive(name, RecordType::Ptr))
    {
        Ok(response) => {
            let names = response
                .answers
                .iter()
                .filter_map(|record| match &record.data {
                    RData::Name(RecordType::Ptr, name) => Some(name.to_string()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(",");
            if names.is_empty() {
                line.to_owned()
            } else {
                format!("{left}={names}{right}")
            }
        }
        Err(error) => format!("{left}:{}{right}", error.to_string().replace(' ', "-")),
    }
}
```
