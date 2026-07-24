---
type: "code-fragment"
fragment_id: "rgbdns-frag-8d1f39317994"
source_path: "src/multilog.rs"
code_note: "DNS from First Principles/Code/src/multilog.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "parse_bounded"
kind: "fn"
start_line: 62
end_line: 72
---

# parse_bounded

- Fragment ID: `rgbdns-frag-8d1f39317994`
- Source file: [[DNS from First Principles/Code/src/multilog.rs.source|src/multilog.rs]]
- Lines: 62-72
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-8d1f39317994", "codeNote": "DNS from First Principles/Code/src/multilog.rs.source", "heading": "rgbdns-frag-8d1f39317994: fn parse_bounded", "sourcePath": "src/multilog.rs", "startLine": 62, "endLine": 72}
```

## Excerpt

<span id="rgbdns-frag-8d1f39317994" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8d1f39317994: fn parse_bounded

```rust
fn parse_bounded<T>(value: &str, minimum: T, maximum: T, label: &str) -> Result<T, String>
where
    T: Copy + Ord + std::str::FromStr,
{
    let value = value.parse::<T>().map_err(|_| format!("invalid {label}"))?;
    if !(minimum..=maximum).contains(&value) {
        return Err(format!("{label} is outside the supported range"));
    }
    Ok(value)
}

```
