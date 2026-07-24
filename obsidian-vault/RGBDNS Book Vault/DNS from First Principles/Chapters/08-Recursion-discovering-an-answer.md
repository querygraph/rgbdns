---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Recursion: discovering an answer

## Iteration from the root

A recursive resolver turns one client request into a bounded sequence of
queries. For `www.example.com. A`, a cold lookup is approximately:

```text
stub → recursive resolver
          ├─ root:       who serves com?
          ├─ com server: who serves example.com?
          └─ example:    what is www.example.com A?
     ← final answer
```

The resolver follows referrals, resolves nameserver addresses when glue is
insufficient, handles aliases, retries servers and transports, and detects
loops. It caches useful RRsets so later clients may skip most of this path.

Root hints are not answers to every name. They are bootstrap addresses for
reaching the root authority. They need periodic maintenance because the set
can change, though names and anycast make changes infrequent.

## The cache is part of correctness

A cache key includes at least name, type, and class. A cached positive RRset
expires according to TTL. Negative results also have bounded lifetimes derived
from SOA data. Delegation and nameserver-address caches help the iterative
algorithm navigate efficiently.

Capacity is as important as time. An attacker can generate endless distinct
names. An unbounded cache converts traffic into memory exhaustion. A practical
resolver bounds response cache bytes, nameserver cache entries, recursion
depth, referral work, packet sizes, concurrent operations, and timeouts.

`src/bin/dnscache.rs` uses Hickory’s recursive zone handler inside rgbdns’s
process and policy shell. It configures:

- randomized query-name letter case;
- a bounded response cache, defaulting to 16 MiB;
- a bounded nameserver cache;
- bounded ordinary and nameserver recursion depth;
- a 1232-byte EDNS payload;
- UDP and TCP listeners;
- loopback-only clients by default, expanded through `ALLOW_NETS`.

Configuration values are parsed with explicit minimums and maximums. A typo
such as an enormous cache size fails startup rather than silently allocating an
operator’s mistake.

## Forward zones and djbdns roots

Private namespaces and split DNS often need selected suffixes sent to specific
servers. rgbdns reads forward-zone configuration from the environment and the
djbdns-style `ROOT/servers` directory. The filename identifies a suffix and
the file lists bounded server addresses.

The special `servers/@` file represents root servers. Hickory consumes a root
hints file in master-file syntax, so `PreparedRoots` translates djbdns’s plain
address list into a private temporary file. Creation uses restrictive
permissions and cleanup occurs when the prepared object is dropped. This
adapter preserves the external configuration contract without weakening the
library boundary.

Forwarded private zones disable strict case-randomization response matching
because legacy authorities may canonicalize owner case. They retain TCP retry
and a bounded cache. This is a scoped compatibility decision, not a global
removal of query hardening.

## Generated Code Fragment Index

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-0c2a1a2a636a", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "heading": "rgbdns-frag-0c2a1a2a636a: fn main", "sourcePath": "src/bin/dnscache.rs", "startLine": 21, "endLine": 35}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f9c2e38b98db", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "heading": "rgbdns-frag-f9c2e38b98db: fn run", "sourcePath": "src/bin/dnscache.rs", "startLine": 36, "endLine": 116}
```

```rgbdns-fragment
{"id": "rgbdns-frag-c4c75c363342", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "heading": "rgbdns-frag-c4c75c363342: fn bounded_env_usize", "sourcePath": "src/bin/dnscache.rs", "startLine": 143, "endLine": 151}
```

```rgbdns-fragment
{"id": "rgbdns-frag-93419b6c9023", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "heading": "rgbdns-frag-93419b6c9023: fn bounded_env_u8", "sourcePath": "src/bin/dnscache.rs", "startLine": 152, "endLine": 160}
```

```rgbdns-fragment
{"id": "rgbdns-frag-985f21d17a56", "codeNote": "DNS from First Principles/Code/src/bin/dnscache.rs.source", "heading": "rgbdns-frag-985f21d17a56: fn bounded_env", "sourcePath": "src/bin/dnscache.rs", "startLine": 161, "endLine": 180}
```

```rgbdns-fragment
{"id": "rgbdns-frag-3790fed38817", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-3790fed38817: const MAX_ROOTS_FILE", "sourcePath": "src/dnscache_config.rs", "startLine": 12, "endLine": 12}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f59cd49b7a3e", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-f59cd49b7a3e: const MAX_ROOT_ADDRESSES", "sourcePath": "src/dnscache_config.rs", "startLine": 13, "endLine": 13}
```

```rgbdns-fragment
{"id": "rgbdns-frag-e2e70010249a", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-e2e70010249a: const MAX_FORWARD_ZONES", "sourcePath": "src/dnscache_config.rs", "startLine": 14, "endLine": 16}
```

```rgbdns-fragment
{"id": "rgbdns-frag-128d48d8a73e", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-128d48d8a73e: struct ForwardZone", "sourcePath": "src/dnscache_config.rs", "startLine": 17, "endLine": 26}
```

```rgbdns-fragment
{"id": "rgbdns-frag-c527bbc0d8ea", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-c527bbc0d8ea: struct PreparedRoots", "sourcePath": "src/dnscache_config.rs", "startLine": 27, "endLine": 31}
```

```rgbdns-fragment
{"id": "rgbdns-frag-2ed2d81c2acf", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-2ed2d81c2acf: impl PreparedRoots", "sourcePath": "src/dnscache_config.rs", "startLine": 32, "endLine": 32}
```

```rgbdns-fragment
{"id": "rgbdns-frag-1c885999a2f2", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-1c885999a2f2: fn from_environment", "sourcePath": "src/dnscache_config.rs", "startLine": 33, "endLine": 45}
```
