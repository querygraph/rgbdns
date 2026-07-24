---
type: "code-fragment"
fragment_id: "rgbdns-frag-d7301c7d3da0"
source_path: "src/setuidgid.rs"
code_note: "DNS from First Principles/Code/src/setuidgid.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "exec"
kind: "fn"
start_line: 78
end_line: 86
---

# exec

- Fragment ID: `rgbdns-frag-d7301c7d3da0`
- Source file: [[DNS from First Principles/Code/src/setuidgid.rs.source|src/setuidgid.rs]]
- Lines: 78-86
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d7301c7d3da0", "codeNote": "DNS from First Principles/Code/src/setuidgid.rs.source", "heading": "rgbdns-frag-d7301c7d3da0: fn exec", "sourcePath": "src/setuidgid.rs", "startLine": 78, "endLine": 86}
```

## Excerpt

<span id="rgbdns-frag-d7301c7d3da0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d7301c7d3da0: fn exec

```rust
pub fn exec(program: &CStr, arguments: &[CString]) -> Result<(), String> {
    let references = arguments.iter().map(CString::as_c_str).collect::<Vec<_>>();
    match nix::unistd::execvp(program, &references) {
        Ok(never) => match never {},
        Err(error) => Err(error.to_string()),
    }
}

#[cfg(test)]
```
