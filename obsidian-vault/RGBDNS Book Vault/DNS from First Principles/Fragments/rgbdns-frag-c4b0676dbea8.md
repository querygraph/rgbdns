---
type: "code-fragment"
fragment_id: "rgbdns-frag-c4b0676dbea8"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "resolve"
kind: "fn"
start_line: 44
end_line: 95
---

# resolve

- Fragment ID: `rgbdns-frag-c4b0676dbea8`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 44-95
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c4b0676dbea8", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "heading": "rgbdns-frag-c4b0676dbea8: fn resolve", "sourcePath": "src/aname.rs", "startLine": 44, "endLine": 95}
```

## Excerpt

<span id="rgbdns-frag-c4b0676dbea8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c4b0676dbea8: fn resolve

```rust
    pub(crate) fn resolve(
        &self,
        owner: &Name,
        target: &Name,
        record_type: RecordType,
        ttl_limit: u32,
    ) -> Result<Vec<Record>> {
        if !matches!(record_type, RecordType::A | RecordType::Aaaa) {
            return Ok(Vec::new());
        }
        let key = CacheKey {
            target: target.clone(),
            record_type,
        };
        let now = Instant::now();
        if let Some(entry) = self
            .cache
            .lock()
            .map_err(|_| Error::Format("ANAME cache lock poisoned"))?
            .get(&key)
            .filter(|entry| entry.expires > now)
            .cloned()
        {
            return Ok(records(owner, entry, now, ttl_limit));
        }

        let response = client::query(target.clone(), record_type, true, &self.servers)?;
        if !matches!(response.flags & 0x000f, 0 | 3) {
            return Err(Error::Format("ANAME upstream resolver returned an error"));
        }
        let (data, upstream_ttl) = addresses(&response.answers, target, record_type)?;
        let negative_ttl = response
            .authorities
            .iter()
            .filter_map(|record| match record.data {
                RData::Soa { minimum, .. } => Some(record.ttl.min(minimum)),
                _ => None,
            })
            .min();
        let ttl = upstream_ttl.or(negative_ttl).unwrap_or(NEGATIVE_TTL).max(1);
        let entry = CacheEntry {
            data,
            expires: now + Duration::from_secs(u64::from(ttl)),
        };
        self.cache
            .lock()
            .map_err(|_| Error::Format("ANAME cache lock poisoned"))?
            .insert(key, entry.clone());
        Ok(records(owner, entry, now, ttl_limit))
    }
}

```
