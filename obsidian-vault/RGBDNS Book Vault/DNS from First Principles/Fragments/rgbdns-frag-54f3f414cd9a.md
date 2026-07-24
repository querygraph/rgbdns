---
type: "code-fragment"
fragment_id: "rgbdns-frag-54f3f414cd9a"
source_path: "src/bin/setuidgid.rs"
code_note: "DNS from First Principles/Code/src/bin/setuidgid.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "setuidgid"
symbol: "run"
kind: "fn"
start_line: 10
end_line: 21
---

# run

- Fragment ID: `rgbdns-frag-54f3f414cd9a`
- Source file: [[DNS from First Principles/Code/src/bin/setuidgid.rs.source|src/bin/setuidgid.rs]]
- Lines: 10-21
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/setuidgid|setuidgid]]

```rgbdns-fragment
{"id": "rgbdns-frag-54f3f414cd9a", "codeNote": "DNS from First Principles/Code/src/bin/setuidgid.rs.source", "heading": "rgbdns-frag-54f3f414cd9a: fn run", "sourcePath": "src/bin/setuidgid.rs", "startLine": 10, "endLine": 21}
```

## Excerpt

<span id="rgbdns-frag-54f3f414cd9a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-54f3f414cd9a: fn run

```rust
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
```
