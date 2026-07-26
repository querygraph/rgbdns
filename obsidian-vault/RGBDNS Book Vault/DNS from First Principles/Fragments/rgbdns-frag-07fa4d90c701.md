---
type: "code-fragment"
fragment_id: "rgbdns-frag-07fa4d90c701"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "records"
kind: "fn"
start_line: 149
end_line: 167
---

# records

- Fragment ID: `rgbdns-frag-07fa4d90c701`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 149-167
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-07fa4d90c701", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "heading": "rgbdns-frag-07fa4d90c701: fn records", "sourcePath": "src/aname.rs", "startLine": 149, "endLine": 167}
```

## Excerpt

<span id="rgbdns-frag-07fa4d90c701" class="rgbdns-fragment-target"></span>
### rgbdns-frag-07fa4d90c701: fn records

```rust
fn records(owner: &Name, entry: CacheEntry, now: Instant, ttl_limit: u32) -> Vec<Record> {
    let ttl = entry
        .expires
        .saturating_duration_since(now)
        .as_secs()
        .clamp(1, u64::from(u32::MAX)) as u32;
    let ttl = ttl.min(ttl_limit);
    entry
        .data
        .into_iter()
        .map(|data| Record {
            name: owner.clone(),
            ttl,
            data,
        })
        .collect()
}

#[cfg(test)]
```
