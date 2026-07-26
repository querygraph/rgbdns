---
type: "code-file"
source_path: "src/bin/setuidgid.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "setuidgid"
line_count: 25
fragment_count: 3
rgbdns_commit: "79502939"
---

# src/bin/setuidgid.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/setuidgid|setuidgid]]
- Source path: `src/bin/setuidgid.rs`
- Lines: 25
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-7346526d2869|main]]: lines 2-9
- [[DNS from First Principles/Fragments/rgbdns-frag-54f3f414cd9a|run]]: lines 10-21
- [[DNS from First Principles/Fragments/rgbdns-frag-41648bb6489a|main]]: lines 22-25

## Full Source

```rust
#[cfg(unix)]
fn main() {
    if let Err(error) = run() {
        eprintln!("setuidgid: fatal: {error}");
        std::process::exit(111);
    }
}

#[cfg(unix)]
fn run() -> Result<(), String> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let account = arguments
        .first()
        .ok_or_else(|| "usage: setuidgid account program [arg ...]".to_owned())?;
    let identity = rgbdns::setuidgid::resolve(account)?;
    let (program, arguments) = rgbdns::setuidgid::command(&arguments[1..])?;
    rgbdns::setuidgid::drop_privileges(&identity)?;
    rgbdns::setuidgid::exec(&program, &arguments)
}

#[cfg(not(unix))]
fn main() {
    eprintln!("setuidgid: fatal: this platform does not support Unix identities");
    std::process::exit(111);
}
```
