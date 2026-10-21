---
type: "code-fragment"
fragment_id: "rgbdns-frag-330c4abefa00"
source_path: "src/bin/aname-materialize.rs"
code_note: "DNS from First Principles/Code/src/bin/aname-materialize.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "aname-materialize"
symbol: "main"
kind: "fn"
start_line: 4
end_line: 31
---

# main

- Fragment ID: `rgbdns-frag-330c4abefa00`
- Source file: [[DNS from First Principles/Code/src/bin/aname-materialize.rs.source|src/bin/aname-materialize.rs]]
- Lines: 4-31
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/aname-materialize|aname-materialize]]

```rgbdns-fragment
{"id": "rgbdns-frag-330c4abefa00", "codeNote": "DNS from First Principles/Code/src/bin/aname-materialize.rs.source", "sourcePath": "src/bin/aname-materialize.rs", "startLine": 4, "endLine": 31}
```

## Excerpt

<span id="rgbdns-frag-330c4abefa00" class="rgbdns-fragment-target"></span>
### rgbdns-frag-330c4abefa00: fn main

```rust
fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() > 3 {
        eprintln!("usage: aname-materialize [data [data.materialized [dnssec]]]");
        return ExitCode::from(100);
    }
    let input = Path::new(arguments.first().map_or("data", String::as_str));
    let output = Path::new(arguments.get(1).map_or("data.materialized", String::as_str));
    let result = if let Some(policy) = arguments.get(2) {
        dnssec::materialize_policy_file(input, Path::new(policy), output)
    } else {
        dnssec::materialize_file(input, output)
    };
    match result {
        Ok(Some(expires)) => {
            println!("{expires}\tok");
            ExitCode::SUCCESS
        }
        Ok(None) => {
            println!("-\tok");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("aname-materialize: {error}");
            ExitCode::from(111)
        }
    }
}
```
