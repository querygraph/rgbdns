---
type: "code-fragment"
fragment_id: "rgbdns-frag-8ab25cad4553"
source_path: "tests/cdb_golden.rs"
code_note: "DNS from First Principles/Code/tests/cdb_golden.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "expected"
kind: "fn"
start_line: 49
end_line: 55
---

# expected

- Fragment ID: `rgbdns-frag-8ab25cad4553`
- Source file: [[DNS from First Principles/Code/tests/cdb_golden.rs.source|tests/cdb_golden.rs]]
- Lines: 49-55
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-8ab25cad4553", "codeNote": "DNS from First Principles/Code/tests/cdb_golden.rs.source", "heading": "rgbdns-frag-8ab25cad4553: fn expected", "sourcePath": "tests/cdb_golden.rs", "startLine": 49, "endLine": 55}
```

## Excerpt

<span id="rgbdns-frag-8ab25cad4553" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8ab25cad4553: fn expected

```rust
fn expected(contents: &str) -> Vec<String> {
    let mut lines = contents.lines().map(str::to_owned).collect::<Vec<_>>();
    lines.sort();
    lines
}

#[test]
```
