---
type: "code-fragment"
fragment_id: "rgbdns-frag-5b3e2e25c63a"
source_path: "src/bin/rgbsec-data.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbsec-data.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbsec-data"
symbol: "main"
kind: "fn"
start_line: 4
end_line: 19
---

# main

- Fragment ID: `rgbdns-frag-5b3e2e25c63a`
- Source file: [[DNS from First Principles/Code/src/bin/rgbsec-data.rs.source|src/bin/rgbsec-data.rs]]
- Lines: 4-19
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbsec-data|rgbsec-data]]

```rgbdns-fragment
{"id": "rgbdns-frag-5b3e2e25c63a", "codeNote": "DNS from First Principles/Code/src/bin/rgbsec-data.rs.source", "sourcePath": "src/bin/rgbsec-data.rs", "startLine": 4, "endLine": 19}
```

## Excerpt

<span id="rgbdns-frag-5b3e2e25c63a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5b3e2e25c63a: fn main

```rust
fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() > 2 {
        eprintln!("usage: rgbsec-data [data [data.cdb]]");
        return ExitCode::from(100);
    }
    let input = Path::new(arguments.first().map_or("data", String::as_str));
    let output = Path::new(arguments.get(1).map_or("data.cdb", String::as_str));
    match dnssec::compile_file(input, Path::new("dnssec"), output) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("rgbsec-data: {error}");
            ExitCode::from(111)
        }
    }
}
```
