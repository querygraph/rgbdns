---
type: "code-fragment"
fragment_id: "rgbdns-frag-d642db76538b"
source_path: "src/setuidgid.rs"
code_note: "DNS from First Principles/Code/src/setuidgid.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "validates_exec_arguments"
kind: "fn"
start_line: 100
end_line: 107
---

# validates_exec_arguments

- Fragment ID: `rgbdns-frag-d642db76538b`
- Source file: [[DNS from First Principles/Code/src/setuidgid.rs.source|src/setuidgid.rs]]
- Lines: 100-107
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d642db76538b", "codeNote": "DNS from First Principles/Code/src/setuidgid.rs.source", "heading": "rgbdns-frag-d642db76538b: fn validates_exec_arguments", "sourcePath": "src/setuidgid.rs", "startLine": 100, "endLine": 107}
```

## Excerpt

<span id="rgbdns-frag-d642db76538b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d642db76538b: fn validates_exec_arguments

```rust
    fn validates_exec_arguments() {
        let (program, arguments) = command(&["printf".into(), "ok".into()]).unwrap();
        assert_eq!(program.as_bytes(), b"printf");
        assert_eq!(arguments.len(), 2);
        assert!(command(&[]).is_err());
        assert!(command(&["bad\0program".into()]).is_err());
    }
}
```
