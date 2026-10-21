---
type: "code-fragment"
fragment_id: "rgbdns-frag-3ec05933a0fe"
source_path: "src/bin/acme-materialize.rs"
code_note: "DNS from First Principles/Code/src/bin/acme-materialize.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "acme-materialize"
symbol: "main"
kind: "fn"
start_line: 4
end_line: 22
---

# main

- Fragment ID: `rgbdns-frag-3ec05933a0fe`
- Source file: [[DNS from First Principles/Code/src/bin/acme-materialize.rs.source|src/bin/acme-materialize.rs]]
- Lines: 4-22
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/acme-materialize|acme-materialize]]

```rgbdns-fragment
{"id": "rgbdns-frag-3ec05933a0fe", "codeNote": "DNS from First Principles/Code/src/bin/acme-materialize.rs.source", "sourcePath": "src/bin/acme-materialize.rs", "startLine": 4, "endLine": 22}
```

## Excerpt

<span id="rgbdns-frag-3ec05933a0fe" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3ec05933a0fe: fn main

```rust
fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 4 {
        eprintln!("usage: acme-materialize data acme-config state-directory output");
        return ExitCode::from(100);
    }
    match acme::materialize_state(
        Path::new(&arguments[0]),
        Path::new(&arguments[1]),
        Path::new(&arguments[2]),
        Path::new(&arguments[3]),
    ) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("acme-materialize: {error}");
            ExitCode::from(111)
        }
    }
}
```
