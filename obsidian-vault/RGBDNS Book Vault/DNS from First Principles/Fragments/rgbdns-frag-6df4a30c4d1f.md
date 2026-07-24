---
type: "code-fragment"
fragment_id: "rgbdns-frag-6df4a30c4d1f"
source_path: "src/bin/axfr-get.rs"
code_note: "DNS from First Principles/Code/src/bin/axfr-get.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "axfr-get"
symbol: "run"
kind: "fn"
start_line: 11
end_line: 37
---

# run

- Fragment ID: `rgbdns-frag-6df4a30c4d1f`
- Source file: [[DNS from First Principles/Code/src/bin/axfr-get.rs.source|src/bin/axfr-get.rs]]
- Lines: 11-37
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/axfr-get|axfr-get]]

```rgbdns-fragment
{"id": "rgbdns-frag-6df4a30c4d1f", "codeNote": "DNS from First Principles/Code/src/bin/axfr-get.rs.source", "heading": "rgbdns-frag-6df4a30c4d1f: fn run", "sourcePath": "src/bin/axfr-get.rs", "startLine": 11, "endLine": 37}
```

## Excerpt

<span id="rgbdns-frag-6df4a30c4d1f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6df4a30c4d1f: fn run

```rust
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = std::env::args().skip(1);
    let zone: Name = arguments
        .next()
        .ok_or("usage: axfr-get zone server[:port] output temporary")?
        .parse()?;
    let server_text = arguments
        .next()
        .ok_or("usage: axfr-get zone server[:port] output temporary")?;
    let server: SocketAddr = if server_text.contains(':') {
        server_text.parse()?
    } else {
        format!("{server_text}:53").parse()?
    };
    let output = arguments
        .next()
        .ok_or("usage: axfr-get zone server[:port] output temporary")?;
    let temporary = arguments
        .next()
        .ok_or("usage: axfr-get zone server[:port] output temporary")?;
    if arguments.next().is_some() {
        return Err("usage: axfr-get zone server[:port] output temporary".into());
    }
    let records = axfr::fetch(server, zone)?;
    axfr::write_tinydns(&records, Path::new(&output), Path::new(&temporary))?;
    Ok(())
}
```
