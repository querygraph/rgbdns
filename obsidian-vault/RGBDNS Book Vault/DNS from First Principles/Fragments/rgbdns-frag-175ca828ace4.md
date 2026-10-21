---
type: "code-fragment"
fragment_id: "rgbdns-frag-175ca828ace4"
source_path: "src/bin/rgbsec-sign.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbsec-sign.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbsec-sign"
symbol: "main"
kind: "fn"
start_line: 4
end_line: 19
---

# main

- Fragment ID: `rgbdns-frag-175ca828ace4`
- Source file: [[DNS from First Principles/Code/src/bin/rgbsec-sign.rs.source|src/bin/rgbsec-sign.rs]]
- Lines: 4-19
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbsec-sign|rgbsec-sign]]

```rgbdns-fragment
{"id": "rgbdns-frag-175ca828ace4", "codeNote": "DNS from First Principles/Code/src/bin/rgbsec-sign.rs.source", "sourcePath": "src/bin/rgbsec-sign.rs", "startLine": 4, "endLine": 19}
```

## Excerpt

<span id="rgbdns-frag-175ca828ace4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-175ca828ace4: fn main

```rust
fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() > 2 {
        eprintln!("usage: rgbsec-sign [data [data.signed]]");
        return ExitCode::from(100);
    }
    let input = Path::new(arguments.first().map_or("data", String::as_str));
    let output = Path::new(arguments.get(1).map_or("data.signed", String::as_str));
    match dnssec::sign_file(input, Path::new("dnssec"), output) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("rgbsec-sign: {error}");
            ExitCode::from(111)
        }
    }
}
```
