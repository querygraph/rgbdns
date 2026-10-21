---
type: "code-fragment"
fragment_id: "rgbdns-frag-9b7c922dda7c"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "materialize_policy_file"
kind: "fn"
start_line: 617
end_line: 625
---

# materialize_policy_file

- Fragment ID: `rgbdns-frag-9b7c922dda7c`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 617-625
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9b7c922dda7c", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 617, "endLine": 625}
```

## Excerpt

<span id="rgbdns-frag-9b7c922dda7c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9b7c922dda7c: fn materialize_policy_file

```rust
pub fn materialize_policy_file(
    input: &Path,
    policy_file: &Path,
    output: &Path,
) -> Result<Option<u64>> {
    let policies = read_policies(policy_file)?;
    materialize_file_with_policies(input, Some(&policies), output)
}

```
