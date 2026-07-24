---
type: "code-fragment"
fragment_id: "rgbdns-frag-ae456975540f"
source_path: "src/bin/tinydns-edit.rs"
code_note: "DNS from First Principles/Code/src/bin/tinydns-edit.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "tinydns-edit"
symbol: "run"
kind: "fn"
start_line: 14
end_line: 33
---

# run

- Fragment ID: `rgbdns-frag-ae456975540f`
- Source file: [[DNS from First Principles/Code/src/bin/tinydns-edit.rs.source|src/bin/tinydns-edit.rs]]
- Lines: 14-33
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tinydns-edit|tinydns-edit]]

```rgbdns-fragment
{"id": "rgbdns-frag-ae456975540f", "codeNote": "DNS from First Principles/Code/src/bin/tinydns-edit.rs.source", "heading": "rgbdns-frag-ae456975540f: fn run", "sourcePath": "src/bin/tinydns-edit.rs", "startLine": 14, "endLine": 33}
```

## Excerpt

<span id="rgbdns-frag-ae456975540f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ae456975540f: fn run

```rust
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 6 || arguments[2] != "add" {
        return Err("usage: tinydns-edit data data.new add \
             [ns|childns|host|alias|mx|host6|alias6] domain address"
            .into());
    }
    tinydns_edit::add(
        Path::new(&arguments[0]),
        Path::new(&arguments[1]),
        tinydns_edit::Mode::parse(&arguments[3])?,
        arguments[4].parse::<Name>()?,
        if matches!(arguments[3].as_str(), "host6" | "alias6") {
            tinydns_edit::Address::V6(arguments[5].parse::<Ipv6Addr>()?)
        } else {
            tinydns_edit::Address::V4(arguments[5].parse::<Ipv4Addr>()?)
        },
    )?;
    Ok(())
}
```
