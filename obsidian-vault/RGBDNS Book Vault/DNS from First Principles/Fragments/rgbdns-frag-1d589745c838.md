---
type: "code-fragment"
fragment_id: "rgbdns-frag-1d589745c838"
source_path: "src/pick.rs"
code_note: "DNS from First Principles/Code/src/pick.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "shuffle"
kind: "fn"
start_line: 210
end_line: 228
---

# shuffle

- Fragment ID: `rgbdns-frag-1d589745c838`
- Source file: [[DNS from First Principles/Code/src/pick.rs.source|src/pick.rs]]
- Lines: 210-228
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1d589745c838", "codeNote": "DNS from First Principles/Code/src/pick.rs.source", "sourcePath": "src/pick.rs", "startLine": 210, "endLine": 228}
```

## Excerpt

<span id="rgbdns-frag-1d589745c838" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1d589745c838: fn shuffle

```rust
fn shuffle<T>(values: &mut [T]) -> Result<()> {
    for end in (1..values.len()).rev() {
        let upper = end as u64 + 1;
        let threshold = u64::MAX - (u64::MAX % upper);
        let index = loop {
            let mut bytes = [0; 8];
            getrandom::fill(&mut bytes)
                .map_err(|_| Error::Io(std::io::Error::other("OS randomness unavailable")))?;
            let value = u64::from_ne_bytes(bytes);
            if value < threshold {
                break (value % upper) as usize;
            }
        };
        values.swap(end, index);
    }
    Ok(())
}

#[cfg(test)]
```
