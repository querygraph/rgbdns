---
type: "code-file"
source_path: "src/aname.rs"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
line_count: 212
fragment_count: 15
rgbdns_commit: "79502939"
---

# src/aname.rs

- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]
- Source path: `src/aname.rs`
- Lines: 212
- Summary: Private ANAME resolution and bounded address caching.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-0eb12106aceb|MAX_CHAIN]]: lines 11-11
- [[DNS from First Principles/Fragments/rgbdns-frag-82b35b6804ab|NEGATIVE_TTL]]: lines 12-12
- [[DNS from First Principles/Fragments/rgbdns-frag-3d0508ce22a6|MAX_ADDRESSES]]: lines 13-15
- [[DNS from First Principles/Fragments/rgbdns-frag-163088533e30|CacheKey]]: lines 16-21
- [[DNS from First Principles/Fragments/rgbdns-frag-57473282a4ff|CacheEntry]]: lines 22-26
- [[DNS from First Principles/Fragments/rgbdns-frag-3570b5155e9d|Resolver]]: lines 27-31
- [[DNS from First Principles/Fragments/rgbdns-frag-a5b88df041d6|Resolver]]: lines 32-32
- [[DNS from First Principles/Fragments/rgbdns-frag-583cdf24733b|from_system]]: lines 33-36
- [[DNS from First Principles/Fragments/rgbdns-frag-ab345aca2e36|new]]: lines 37-43
- [[DNS from First Principles/Fragments/rgbdns-frag-c4b0676dbea8|resolve]]: lines 44-95
- [[DNS from First Principles/Fragments/rgbdns-frag-b8118dfc2f5e|addresses]]: lines 96-148
- [[DNS from First Principles/Fragments/rgbdns-frag-07fa4d90c701|records]]: lines 149-167
- [[DNS from First Principles/Fragments/rgbdns-frag-bca7f405b15b|tests]]: lines 168-172
- [[DNS from First Principles/Fragments/rgbdns-frag-9f56afdfcc8f|follows_a_bounded_cname_chain_and_preserves_addresses]]: lines 173-193
- [[DNS from First Principles/Fragments/rgbdns-frag-d418530011a5|separates_address_families_and_rejects_loops]]: lines 194-212

## Full Source

```rust
//! Private ANAME resolution and bounded address caching.

use crate::{Error, Name, RData, Record, RecordType, Result, client};
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::Mutex,
    time::{Duration, Instant},
};

const MAX_CHAIN: usize = 16;
const NEGATIVE_TTL: u32 = 60;
const MAX_ADDRESSES: usize = 64;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct CacheKey {
    target: Name,
    record_type: RecordType,
}

#[derive(Clone, Debug)]
struct CacheEntry {
    data: Vec<RData>,
    expires: Instant,
}

pub(crate) struct Resolver {
    servers: Vec<SocketAddr>,
    cache: Mutex<HashMap<CacheKey, CacheEntry>>,
}

impl Resolver {
    pub(crate) fn from_system() -> Result<Self> {
        Ok(Self::new(client::servers()?))
    }

    pub(crate) fn new(servers: Vec<SocketAddr>) -> Self {
        Self {
            servers,
            cache: Mutex::new(HashMap::new()),
        }
    }

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

fn addresses(
    answers: &[Record],
    target: &Name,
    record_type: RecordType,
) -> Result<(Vec<RData>, Option<u32>)> {
    let mut current = target.clone();
    let mut visited = HashSet::new();
    let mut ttl = None;
    for _ in 0..MAX_CHAIN {
        if !visited.insert(current.clone()) {
            return Err(Error::Format("ANAME target contains a CNAME loop"));
        }
        let values = answers
            .iter()
            .filter(|record| record.name == current && record.rr_type() == record_type)
            .take(MAX_ADDRESSES + 1)
            .collect::<Vec<_>>();
        if values.len() > MAX_ADDRESSES {
            return Err(Error::Format("ANAME target has too many addresses"));
        }
        if !values.is_empty() {
            for record in &values {
                ttl = Some(ttl.map_or(record.ttl, |value: u32| value.min(record.ttl)));
            }
            return Ok((
                values
                    .into_iter()
                    .map(|record| record.data.clone())
                    .collect(),
                ttl,
            ));
        }
        let aliases = answers
            .iter()
            .filter_map(|record| match &record.data {
                RData::Name(RecordType::Cname, next) if record.name == current => {
                    Some((next, record.ttl))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        if aliases.is_empty() {
            return Ok((Vec::new(), ttl));
        }
        if aliases.iter().any(|(next, _)| *next != aliases[0].0) {
            return Err(Error::Format("ANAME target has conflicting CNAME answers"));
        }
        ttl = Some(ttl.map_or(aliases[0].1, |value: u32| value.min(aliases[0].1)));
        current = aliases[0].0.clone();
    }
    Err(Error::Format("ANAME target CNAME chain is too long"))
}

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
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn follows_a_bounded_cname_chain_and_preserves_addresses() {
        let first: Name = "one.example".parse().unwrap();
        let second: Name = "two.example".parse().unwrap();
        let answers = vec![
            Record {
                name: first.clone(),
                ttl: 90,
                data: RData::Name(RecordType::Cname, second.clone()),
            },
            Record {
                name: second,
                ttl: 60,
                data: RData::A(Ipv4Addr::new(192, 0, 2, 4)),
            },
        ];
        let (data, ttl) = addresses(&answers, &first, RecordType::A).unwrap();
        assert_eq!(data, [RData::A(Ipv4Addr::new(192, 0, 2, 4))]);
        assert_eq!(ttl, Some(60));
    }

    #[test]
    fn separates_address_families_and_rejects_loops() {
        let name: Name = "loop.example".parse().unwrap();
        let answers = vec![
            Record {
                name: name.clone(),
                ttl: 60,
                data: RData::Aaaa(Ipv6Addr::LOCALHOST),
            },
            Record {
                name: name.clone(),
                ttl: 60,
                data: RData::Name(RecordType::Cname, name.clone()),
            },
        ];
        let (data, _) = addresses(&answers, &name, RecordType::Aaaa).unwrap();
        assert_eq!(data, [RData::Aaaa(Ipv6Addr::LOCALHOST)]);
        assert!(addresses(&answers[1..], &name, RecordType::A).is_err());
    }
}
```
