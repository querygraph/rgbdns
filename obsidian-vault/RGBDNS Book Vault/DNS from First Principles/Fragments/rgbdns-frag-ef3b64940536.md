---
type: "code-fragment"
fragment_id: "rgbdns-frag-ef3b64940536"
source_path: "tests/dnssec_tools.rs"
code_note: "DNS from First Principles/Code/tests/dnssec_tools.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "run"
kind: "fn"
start_line: 10
end_line: 25
---

# run

- Fragment ID: `rgbdns-frag-ef3b64940536`
- Source file: [[DNS from First Principles/Code/tests/dnssec_tools.rs.source|tests/dnssec_tools.rs]]
- Lines: 10-25
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-ef3b64940536", "codeNote": "DNS from First Principles/Code/tests/dnssec_tools.rs.source", "sourcePath": "tests/dnssec_tools.rs", "startLine": 10, "endLine": 25}
```

## Excerpt

<span id="rgbdns-frag-ef3b64940536" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ef3b64940536: fn run

```rust
    fn run(program: &str, arguments: &[&str], directory: &Path) -> Output {
        let output = Command::new(program)
            .args(arguments)
            .current_dir(directory)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{} failed: {}",
            program,
            String::from_utf8_lossy(&output.stderr)
        );
        output
    }

    #[test]
```
