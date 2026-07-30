//! Private ANAME resolution and bounded address caching.

use crate::{Error, Message, Name, RData, Record, RecordType, Result, client};
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::{Condvar, Mutex},
    time::{Duration, Instant},
};

const MAX_CHAIN: usize = 16;
const NEGATIVE_TTL: u32 = 60;
const MAX_ADDRESSES: usize = 64;
const FAILURE_TTL: Duration = Duration::from_secs(5);
const INFLIGHT_WAIT: Duration = Duration::from_secs(30);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct CacheKey {
    target: Name,
    record_type: RecordType,
}

#[derive(Clone, Debug)]
enum CacheEntry {
    Ready { data: Vec<RData>, expires: Instant },
    Loading,
    Failed { expires: Instant },
}

pub(crate) struct Resolver {
    servers: Vec<SocketAddr>,
    cache: Mutex<HashMap<CacheKey, CacheEntry>>,
    cache_changed: Condvar,
}

impl Resolver {
    pub(crate) fn from_system() -> Result<Self> {
        Ok(Self::new(client::servers()?))
    }

    pub(crate) fn new(servers: Vec<SocketAddr>) -> Self {
        Self {
            servers,
            cache: Mutex::new(HashMap::new()),
            cache_changed: Condvar::new(),
        }
    }

    pub(crate) fn resolve(
        &self,
        owner: &Name,
        target: &Name,
        record_type: RecordType,
        ttl_limit: u32,
    ) -> Result<Vec<Record>> {
        self.resolve_with(
            owner,
            target,
            record_type,
            ttl_limit,
            |target, record_type| client::query(target, record_type, true, &self.servers),
        )
    }

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
                Ok(records(owner, data, expires, now, ttl_limit))
            }
            Err(error) => {
                cache.insert(
                    key,
                    CacheEntry::Failed {
                        expires: now + FAILURE_TTL,
                    },
                );
                self.cache_changed.notify_all();
                Err(error)
            }
        }
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
mod tests {
    use super::*;
    use std::{
        net::{Ipv4Addr, Ipv6Addr},
        sync::{
            Arc, Barrier,
            atomic::{AtomicUsize, Ordering},
        },
        thread,
    };

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

    #[test]
    fn concurrent_misses_are_coalesced() {
        let resolver = Arc::new(Resolver::new(Vec::new()));
        let owner: Name = "owner.example".parse().unwrap();
        let target: Name = "target.example".parse().unwrap();
        let calls = Arc::new(AtomicUsize::new(0));
        let barrier = Arc::new(Barrier::new(8));
        let workers = (0..8)
            .map(|_| {
                let resolver = resolver.clone();
                let owner = owner.clone();
                let target = target.clone();
                let calls = calls.clone();
                let barrier = barrier.clone();
                thread::spawn(move || {
                    barrier.wait();
                    resolver
                        .resolve_with(&owner, &target, RecordType::A, 300, |name, _| {
                            calls.fetch_add(1, Ordering::SeqCst);
                            thread::sleep(Duration::from_millis(50));
                            Ok(Message {
                                flags: 0x8000,
                                answers: vec![Record {
                                    name,
                                    ttl: 60,
                                    data: RData::A(Ipv4Addr::new(192, 0, 2, 8)),
                                }],
                                ..Default::default()
                            })
                        })
                        .unwrap()
                })
            })
            .collect::<Vec<_>>();
        for worker in workers {
            let records = worker.join().unwrap();
            assert_eq!(records[0].data, RData::A(Ipv4Addr::new(192, 0, 2, 8)));
        }
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn failures_are_suppressed_briefly() {
        let resolver = Resolver::new(Vec::new());
        let owner: Name = "owner.example".parse().unwrap();
        let target: Name = "target.example".parse().unwrap();
        let calls = AtomicUsize::new(0);
        let first = resolver.resolve_with(&owner, &target, RecordType::A, 300, |_, _| {
            calls.fetch_add(1, Ordering::SeqCst);
            Err(Error::Format("simulated upstream failure"))
        });
        assert!(first.is_err());
        let second = resolver.resolve_with(&owner, &target, RecordType::A, 300, |_, _| {
            calls.fetch_add(1, Ordering::SeqCst);
            Ok(Message::default())
        });
        assert!(second.is_err());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
