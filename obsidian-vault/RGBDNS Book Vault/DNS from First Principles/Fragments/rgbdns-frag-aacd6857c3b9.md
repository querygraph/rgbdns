---
type: "code-fragment"
fragment_id: "rgbdns-frag-aacd6857c3b9"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "records"
kind: "fn"
start_line: 215
end_line: 236
---

# records

- Fragment ID: `rgbdns-frag-aacd6857c3b9`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 215-236
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-aacd6857c3b9", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "sourcePath": "src/aname.rs", "startLine": 215, "endLine": 236}
```

## Excerpt

<span id="rgbdns-frag-aacd6857c3b9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-aacd6857c3b9: fn records

```rust
fn records(
    owner: &Name,
    data: Vec<RData>,
    expires: Instant,
    now: Instant,
    ttl_limit: u32,
) -> Vec<Record> {
    let ttl = expires
        .saturating_duration_since(now)
        .as_secs()
        .clamp(1, u64::from(u32::MAX)) as u32;
    let ttl = ttl.min(ttl_limit);
    data.into_iter()
        .map(|data| Record {
            name: owner.clone(),
            ttl,
            data,
        })
        .collect()
}

#[cfg(test)]
```
