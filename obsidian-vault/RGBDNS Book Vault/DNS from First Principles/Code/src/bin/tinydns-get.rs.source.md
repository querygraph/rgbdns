---
type: "code-file"
source_path: "src/bin/tinydns-get.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "tinydns-get"
line_count: 38
fragment_count: 2
rgbdns_commit: "79502939"
---

# src/bin/tinydns-get.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tinydns-get|tinydns-get]]
- Source path: `src/bin/tinydns-get.rs`
- Lines: 38
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-5d5f0374b6f5|main]]: lines 7-13
- [[DNS from First Principles/Fragments/rgbdns-frag-b8ff2ef5f71d|run]]: lines 14-38

## Full Source

```rust
use rgbdns::{
    packet::{Message, Question, RecordType},
    server,
    zone::Zone,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("tinydns-get: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if !(2..=3).contains(&arguments.len()) {
        return Err(rgbdns::Error::Format("usage: tinydns-get type name [ip]").into());
    }
    let record_type = arguments[0].parse::<RecordType>()?;
    let q = Message {
        id: 1,
        questions: vec![Question {
            name: arguments[1].parse()?,
            qtype: record_type,
            qclass: 1,
        }],
        ..Default::default()
    };
    let zone = Zone::from_file("data")?;
    let query = q.encode()?;
    let wire = if let Some(address) = arguments.get(2) {
        server::respond_from(&zone, &query, 65535, address.parse()?)
    } else {
        server::respond(&zone, &query, 65535)
    }?;
    println!("{:#?}", Message::decode(&wire)?);
    Ok(())
}
```
