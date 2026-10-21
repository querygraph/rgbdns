---
type: "code-fragment"
fragment_id: "rgbdns-frag-c94aedc8bcdb"
source_path: "src/aname.rs"
code_note: "DNS from First Principles/Code/src/aname.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "resolve_with"
kind: "fn"
start_line: 65
end_line: 145
---

# resolve_with

- Fragment ID: `rgbdns-frag-c94aedc8bcdb`
- Source file: [[DNS from First Principles/Code/src/aname.rs.source|src/aname.rs]]
- Lines: 65-145
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c94aedc8bcdb", "codeNote": "DNS from First Principles/Code/src/aname.rs.source", "sourcePath": "src/aname.rs", "startLine": 65, "endLine": 145}
```

## Excerpt

<span id="rgbdns-frag-c94aedc8bcdb" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c94aedc8bcdb: fn resolve_with

```rust
    fn resolve_with<F>(
        &self,
        owner: &Name,
        target: &Name,
        record_type: RecordType,
        ttl_limit: u32,
        query: F,
    ) -> Result<Vec<Record>>
    where
        F: FnOnce(Name, RecordType) -> Result<Message>,
    {
        if !matches!(record_type, RecordType::A | RecordType::Aaaa) {
            return Ok(Vec::new());
        }
        let key = CacheKey {
            target: target.clone(),
            record_type,
        };
        loop {
            let now = Instant::now();
            let mut cache = self
                .cache
                .lock()
                .map_err(|_| Error::Format("ANAME cache lock poisoned"))?;
            match cache.get(&key).cloned() {
                Some(CacheEntry::Ready { data, expires }) if expires > now => {
                    return Ok(records(owner, data, expires, now, ttl_limit));
                }
                Some(CacheEntry::Failed { expires }) if expires > now => {
                    return Err(Error::Format("ANAME target is temporarily suppressed"));
                }
                Some(CacheEntry::Loading) => {
                    let (guard, timeout) = self
                        .cache_changed
                        .wait_timeout(cache, INFLIGHT_WAIT)
                        .map_err(|_| Error::Format("ANAME cache lock poisoned"))?;
                    drop(guard);
                    if timeout.timed_out() {
                        return Err(Error::Format("ANAME target lookup timed out"));
                    }
                    continue;
                }
                _ => {
                    cache.insert(key.clone(), CacheEntry::Loading);
                    break;
                }
            }
        }

        let result = query(target.clone(), record_type).and_then(|response| {
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
            Ok((data, ttl))
        });
        let now = Instant::now();
        let mut cache = self
            .cache
            .lock()
            .map_err(|_| Error::Format("ANAME cache lock poisoned"))?;
        match result {
            Ok((data, ttl)) => {
                let expires = now + Duration::from_secs(u64::from(ttl));
                cache.insert(
                    key,
                    CacheEntry::Ready {
                        data: data.clone(),
                        expires,
                    },
                );
                self.cache_changed.notify_all();
```
