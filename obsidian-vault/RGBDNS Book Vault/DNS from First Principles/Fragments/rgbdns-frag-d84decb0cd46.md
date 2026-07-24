---
type: "code-fragment"
fragment_id: "rgbdns-frag-d84decb0cd46"
source_path: "src/bin/dnsname.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsname.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsname"
symbol: "run"
kind: "fn"
start_line: 11
end_line: 36
---

# run

- Fragment ID: `rgbdns-frag-d84decb0cd46`
- Source file: [[DNS from First Principles/Code/src/bin/dnsname.rs.source|src/bin/dnsname.rs]]
- Lines: 11-36
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsname|dnsname]]

```rgbdns-fragment
{"id": "rgbdns-frag-d84decb0cd46", "codeNote": "DNS from First Principles/Code/src/bin/dnsname.rs.source", "heading": "rgbdns-frag-d84decb0cd46: fn run", "sourcePath": "src/bin/dnsname.rs", "startLine": 11, "endLine": 36}
```

## Excerpt

<span id="rgbdns-frag-d84decb0cd46" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d84decb0cd46: fn run

```rust
fn run() -> rgbdns::Result<()> {
    for argument in std::env::args().skip(1) {
        let address: Ipv4Addr = argument
            .parse()
            .map_err(|_| rgbdns::Error::Format("invalid IPv4 address"))?;
        let octets = address.octets();
        let name = format!(
            "{}.{}.{}.{}.in-addr.arpa",
            octets[3], octets[2], octets[1], octets[0]
        )
        .parse()?;
        let response = client::recursive(name, RecordType::Ptr)?;
        for target in response
            .answers
            .iter()
            .filter_map(|record| match &record.data {
                RData::Name(RecordType::Ptr, target) => Some(target),
                _ => None,
            })
        {
            print!("{target}");
        }
        println!();
    }
    Ok(())
}
```
