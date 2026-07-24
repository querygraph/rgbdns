---
type: "code-fragment"
fragment_id: "rgbdns-frag-985f21d17a56"
source_path: "src/bin/dnscache.rs"
code_note: "DNS from First Principles/Code/src/bin/dnscache.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnscache"
symbol: "bounded_env"
kind: "fn"
start_line: 161
end_line: 180
---

# bounded_env

- Fragment ID: `rgbdns-frag-985f21d17a56`
- Source file: [[DNS from First Principles/Code/src/bin/dnscache.rs.source|src/bin/dnscache.rs]]
- Lines: 161-180
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnscache|dnscache]]

```rgbdns-fragment
{"id": "rgbdns-frag-985f21d17a56", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "heading": "rgbdns-frag-985f21d17a56: fn bounded_env", "sourcePath": "src/bin/dnscache.rs", "startLine": 161, "endLine": 180}
```

## Excerpt

<span id="rgbdns-frag-985f21d17a56" class="rgbdns-fragment-target"></span>
### rgbdns-frag-985f21d17a56: fn bounded_env

```rust
fn bounded_env<T>(
    name: &str,
    default: T,
    minimum: T,
    maximum: T,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: Copy + Ord + std::str::FromStr,
    T::Err: std::error::Error + 'static,
{
    let value = match env::var(name) {
        Ok(value) => value.parse::<T>()?,
        Err(env::VarError::NotPresent) => default,
        Err(error) => return Err(error.into()),
    };
    if !(minimum..=maximum).contains(&value) {
        return Err(format!("{name} is outside the supported range").into());
    }
    Ok(value)
}
```
