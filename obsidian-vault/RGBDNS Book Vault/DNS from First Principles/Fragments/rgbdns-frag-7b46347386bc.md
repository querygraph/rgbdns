---
type: "code-fragment"
fragment_id: "rgbdns-frag-7b46347386bc"
source_path: "src/setuidgid.rs"
code_note: "DNS from First Principles/Code/src/setuidgid.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "command"
kind: "fn"
start_line: 65
end_line: 77
---

# command

- Fragment ID: `rgbdns-frag-7b46347386bc`
- Source file: [[DNS from First Principles/Code/src/setuidgid.rs.source|src/setuidgid.rs]]
- Lines: 65-77
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7b46347386bc", "codeNote": "DNS from First Principles/Code/src/setuidgid.rs.source", "heading": "rgbdns-frag-7b46347386bc: fn command", "sourcePath": "src/setuidgid.rs", "startLine": 65, "endLine": 77}
```

## Excerpt

<span id="rgbdns-frag-7b46347386bc" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7b46347386bc: fn command

```rust
pub fn command(arguments: &[String]) -> Result<(CString, Vec<CString>), String> {
    if arguments.is_empty() {
        return Err("setuidgid requires a program".into());
    }
    let arguments = arguments
        .iter()
        .map(|argument| {
            CString::new(argument.as_bytes()).map_err(|_| "program argument contains NUL".into())
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((arguments[0].clone(), arguments))
}

```
