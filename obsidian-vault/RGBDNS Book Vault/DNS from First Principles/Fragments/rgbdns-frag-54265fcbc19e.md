---
type: "code-fragment"
fragment_id: "rgbdns-frag-54265fcbc19e"
source_path: "src/bin/dnstrace.rs"
code_note: "DNS from First Principles/Code/src/bin/dnstrace.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnstrace"
symbol: "run"
kind: "fn"
start_line: 11
end_line: 91
---

# run

- Fragment ID: `rgbdns-frag-54265fcbc19e`
- Source file: [[DNS from First Principles/Code/src/bin/dnstrace.rs.source|src/bin/dnstrace.rs]]
- Lines: 11-91
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnstrace|dnstrace]]

```rgbdns-fragment
{"id": "rgbdns-frag-54265fcbc19e", "codeNote": "DNS from First Principles/Code/src/bin/dnstrace.rs.source", "heading": "rgbdns-frag-54265fcbc19e: fn run", "sourcePath": "src/bin/dnstrace.rs", "startLine": 11, "endLine": 91}
```

## Excerpt

<span id="rgbdns-frag-54265fcbc19e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-54265fcbc19e: fn run

```rust
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() < 3 {
        return Err("usage: dnstrace type name rootip ...".into());
    }
    let record_type = arguments[0].parse::<RecordType>()?;
    let name = arguments[1].parse::<Name>()?;
    let mut servers = arguments[2..]
        .iter()
        .map(|value| {
            value
                .parse::<IpAddr>()
                .map(|address| SocketAddr::new(address, 53))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let prefix = format!("{} {}: ", record_type.code(), name);
    for depth in 0..64 {
        let server = servers[depth % servers.len()];
        println!("{prefix}QUERY:{server}");
        let response = client::query(name.clone(), record_type, false, &[server])?;
        for record in &response.answers {
            println!("{prefix}ANSWER:{record:?}");
        }
        if !response.answers.is_empty() || response.flags & 15 != 0 {
            println!("{prefix}RCODE:{}", response.flags & 15);
            return Ok(());
        }
        let nameservers = response
            .authorities
            .iter()
            .filter_map(|record| match &record.data {
                RData::Name(RecordType::Ns, target) => {
                    println!("{prefix}NS:{}:{target}", record.name);
                    Some(target.clone())
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        let mut next = Vec::new();
        for record in &response.additionals {
            if nameservers.contains(&record.name) {
                match record.data {
                    RData::A(address) => {
                        println!("{prefix}A:{}:{address}", record.name);
                        next.push(SocketAddr::new(IpAddr::V4(address), 53));
                    }
                    RData::Aaaa(address) => {
                        println!("{prefix}AAAA:{}:{address}", record.name);
                        next.push(SocketAddr::new(IpAddr::V6(address), 53));
                    }
                    _ => {}
                }
            }
        }
        if next.is_empty() {
            for nameserver in nameservers {
                if let Ok(addresses) = client::recursive(nameserver.clone(), RecordType::A) {
                    for record in addresses.answers {
                        if let RData::A(address) = record.data {
                            println!("{prefix}A:{nameserver}:{address}");
                            next.push(SocketAddr::new(IpAddr::V4(address), 53));
                        }
                    }
                }
                if let Ok(addresses) = client::recursive(nameserver.clone(), RecordType::Aaaa) {
                    for record in addresses.answers {
                        if let RData::Aaaa(address) = record.data {
                            println!("{prefix}AAAA:{nameserver}:{address}");
                            next.push(SocketAddr::new(IpAddr::V6(address), 53));
                        }
                    }
                }
            }
        }
        if next.is_empty() {
            return Err("referral contains no reachable IPv4 nameserver".into());
        }
        servers = next;
        if depth == 63 {
            return Err("trace depth limit exceeded".into());
        }
```
