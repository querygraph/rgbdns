---
type: "code-fragment"
fragment_id: "rgbdns-frag-b8ff2ef5f71d"
source_path: "src/bin/tinydns-get.rs"
code_note: "DNS from First Principles/Code/src/bin/tinydns-get.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "tinydns-get"
symbol: "run"
kind: "fn"
start_line: 14
end_line: 38
---

# run

- Fragment ID: `rgbdns-frag-b8ff2ef5f71d`
- Source file: [[DNS from First Principles/Code/src/bin/tinydns-get.rs.source|src/bin/tinydns-get.rs]]
- Lines: 14-38
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tinydns-get|tinydns-get]]

```rgbdns-fragment
{"id": "rgbdns-frag-b8ff2ef5f71d", "codeNote": "DNS from First Principles/Code/src/bin/tinydns-get.rs.source", "heading": "rgbdns-frag-b8ff2ef5f71d: fn run", "sourcePath": "src/bin/tinydns-get.rs", "startLine": 14, "endLine": 38}
```

## Excerpt

<span id="rgbdns-frag-b8ff2ef5f71d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b8ff2ef5f71d: fn run

```rust
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
