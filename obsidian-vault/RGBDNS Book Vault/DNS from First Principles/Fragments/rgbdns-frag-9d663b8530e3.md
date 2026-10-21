---
type: "code-fragment"
fragment_id: "rgbdns-frag-9d663b8530e3"
source_path: "src/bin/rgbsec-ds.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbsec-ds.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbsec-ds"
symbol: "main"
kind: "fn"
start_line: 4
end_line: 20
---

# main

- Fragment ID: `rgbdns-frag-9d663b8530e3`
- Source file: [[DNS from First Principles/Code/src/bin/rgbsec-ds.rs.source|src/bin/rgbsec-ds.rs]]
- Lines: 4-20
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbsec-ds|rgbsec-ds]]

```rgbdns-fragment
{"id": "rgbdns-frag-9d663b8530e3", "codeNote": "DNS from First Principles/Code/src/bin/rgbsec-ds.rs.source", "sourcePath": "src/bin/rgbsec-ds.rs", "startLine": 4, "endLine": 20}
```

## Excerpt

<span id="rgbdns-frag-9d663b8530e3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9d663b8530e3: fn main

```rust
fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 1 {
        eprintln!("usage: rgbsec-ds dnssec-line");
        return ExitCode::from(100);
    }
    match dnssec::Policy::parse(&arguments[0]).and_then(|policy| dnssec::ds_line(&policy)) {
        Ok(line) => {
            println!("{line}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("rgbsec-ds: {error}");
            ExitCode::from(111)
        }
    }
}
```
