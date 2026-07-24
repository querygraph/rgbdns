---
type: "code-fragment"
fragment_id: "rgbdns-frag-9a829bc67682"
source_path: "src/pick.rs"
code_note: "DNS from First Principles/Code/src/pick.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "shuffle"
kind: "fn"
start_line: 208
end_line: 226
---

# shuffle

- Fragment ID: `rgbdns-frag-9a829bc67682`
- Source file: [[DNS from First Principles/Code/src/pick.rs.source|src/pick.rs]]
- Lines: 208-226
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9a829bc67682", "codeNote": "DNS from First Principles/Code/src/pick.rs.source", "heading": "rgbdns-frag-9a829bc67682: fn shuffle", "sourcePath": "src/pick.rs", "startLine": 208, "endLine": 226}
```

## Excerpt

<span id="rgbdns-frag-9a829bc67682" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9a829bc67682: fn shuffle

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
