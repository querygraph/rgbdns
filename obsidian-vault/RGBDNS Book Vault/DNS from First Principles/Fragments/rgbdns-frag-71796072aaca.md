---
type: "code-fragment"
fragment_id: "rgbdns-frag-71796072aaca"
source_path: "src/bin/rgbsec-keygen.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbsec-keygen.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbsec-keygen"
symbol: "main"
kind: "fn"
start_line: 4
end_line: 26
---

# main

- Fragment ID: `rgbdns-frag-71796072aaca`
- Source file: [[DNS from First Principles/Code/src/bin/rgbsec-keygen.rs.source|src/bin/rgbsec-keygen.rs]]
- Lines: 4-26
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbsec-keygen|rgbsec-keygen]]

```rgbdns-fragment
{"id": "rgbdns-frag-71796072aaca", "codeNote": "DNS from First Principles/Code/src/bin/rgbsec-keygen.rs.source", "sourcePath": "src/bin/rgbsec-keygen.rs", "startLine": 4, "endLine": 26}
```

## Excerpt

<span id="rgbdns-frag-71796072aaca" class="rgbdns-fragment-target"></span>
### rgbdns-frag-71796072aaca: fn main

```rust
fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 2 {
        eprintln!("usage: rgbsec-keygen zone keyfile");
        return ExitCode::from(100);
    }
    let result = arguments[0]
        .parse::<Name>()
        .map_err(|error| error.to_string())
        .and_then(|zone| {
            dnssec::generate_key(&zone, Path::new(&arguments[1])).map_err(|error| error.to_string())
        });
    match result {
        Ok(policy) => {
            println!("{}", policy.line());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("rgbsec-keygen: {error}");
            ExitCode::from(111)
        }
    }
}
```
