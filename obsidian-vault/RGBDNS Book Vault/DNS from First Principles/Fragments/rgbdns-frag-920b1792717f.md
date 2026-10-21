---
type: "code-fragment"
fragment_id: "rgbdns-frag-920b1792717f"
source_path: "src/bin/rgbsec-check.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbsec-check.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbsec-check"
symbol: "main"
kind: "fn"
start_line: 4
end_line: 24
---

# main

- Fragment ID: `rgbdns-frag-920b1792717f`
- Source file: [[DNS from First Principles/Code/src/bin/rgbsec-check.rs.source|src/bin/rgbsec-check.rs]]
- Lines: 4-24
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbsec-check|rgbsec-check]]

```rgbdns-fragment
{"id": "rgbdns-frag-920b1792717f", "codeNote": "DNS from First Principles/Code/src/bin/rgbsec-check.rs.source", "sourcePath": "src/bin/rgbsec-check.rs", "startLine": 4, "endLine": 24}
```

## Excerpt

<span id="rgbdns-frag-920b1792717f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-920b1792717f: fn main

```rust
fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() > 2 {
        eprintln!("usage: rgbsec-check [data.signed [dnssec]]");
        return ExitCode::from(100);
    }
    let input = Path::new(arguments.first().map_or("data.signed", String::as_str));
    let policy = Path::new(arguments.get(1).map_or("dnssec", String::as_str));
    match dnssec::check_file(input, policy) {
        Ok(statuses) => {
            for status in statuses {
                println!("{status}");
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("rgbsec-check: {error}");
            ExitCode::from(111)
        }
    }
}
```
