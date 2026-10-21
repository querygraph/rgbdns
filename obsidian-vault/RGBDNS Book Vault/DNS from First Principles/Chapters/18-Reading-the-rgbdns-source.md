---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Reading the rgbdns source

## A path through the code

Read the project in dependency order:

1. `src/name.rs` — the foundational name invariant.
2. `src/packet.rs` — types and bounded wire codec.
3. `src/zone.rs` — tinydns source and authoritative lookup semantics.
4. `src/cdb.rs` — compiled compatibility format.
5. `src/server.rs` — query validation, answer construction, transport limits.
6. `src/client.rs` — stub behavior and TCP fallback.
7. `src/axfr.rs` — streaming zone movement and atomic installation.
8. `src/dnscache_config.rs` and `src/bin/dnscache.rs` — iterative recursion,
   DNSSEC, forwarding, access, and resource policy.
9. `src/rbl.rs`, `src/pick.rs`, `src/wall.rs`, and `src/special.rs` —
   specialized responders.
10. `src/conf.rs`, `src/setuidgid.rs`, `src/multilog.rs`, and `src/tai64.rs` —
    deployment and operations.

The binaries in `src/bin` should then look thin. That is intentional. They
parse the command contract, load configuration, call a library boundary, print
diagnostics, and map fatal errors to the suite’s exit convention.

## Design patterns to carry elsewhere

Several rgbdns choices generalize beyond DNS.

**Parse into valid types.** If an invalid name can circulate as an ordinary
string, every consumer must rediscover validation.

**Bound dimensions independently.** A packet byte limit does not replace a
compression-depth limit; a cache byte limit does not replace a recursion-depth
limit.

**Separate policy from mechanism.** `transport.rs` owns bounded UDP and TCP
mechanics while the authoritative and specialized handlers own answer policy.

**Compile mutable source into immutable serving data.** This gives validation,
atomic rollout, simple readers, and easy rollback.

**Preserve protocol distinctions internally.** A `Lookup` enum prevents
NXDOMAIN, NODATA, referral, and refusal from collapsing into “no records.”

**Run in the foreground.** It composes with old and new supervisors and keeps
signals understandable.

**Treat compatibility files as hostile.** Historical layout fidelity need not
mean historical trust assumptions.

## Generated Code Fragment Index

Each card names an rgbdns fragment. In FirstPair Reader, Show code lists its numbered lines and Open file opens the collocated code note with those lines marked.

```rgbdns-fragment
{"id": "rgbdns-frag-df36f646ddea", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 1, "endLine": 32}
```

```rgbdns-fragment
{"id": "rgbdns-frag-530a102cf907", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 33, "endLine": 48}
```

```rgbdns-fragment
{"id": "rgbdns-frag-087f5fd9bf6b", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 49, "endLine": 111}
```

```rgbdns-fragment
{"id": "rgbdns-frag-c50fc5b27770", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 112, "endLine": 127}
```

```rgbdns-fragment
{"id": "rgbdns-frag-8f6c800e8ac3", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 128, "endLine": 187}
```

```rgbdns-fragment
{"id": "rgbdns-frag-44ef9d3327cb", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 188, "endLine": 207}
```

```rgbdns-fragment
{"id": "rgbdns-frag-d1fb368f56c0", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 208, "endLine": 219}
```

```rgbdns-fragment
{"id": "rgbdns-frag-0764202cbb5b", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "sourcePath": "src/lib.rs", "startLine": 3, "endLine": 3}
```

```rgbdns-fragment
{"id": "rgbdns-frag-d814f58c3808", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "sourcePath": "src/lib.rs", "startLine": 4, "endLine": 4}
```

```rgbdns-fragment
{"id": "rgbdns-frag-137fad6744bd", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "sourcePath": "src/lib.rs", "startLine": 5, "endLine": 5}
```

```rgbdns-fragment
{"id": "rgbdns-frag-56c62ce446e3", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "sourcePath": "src/lib.rs", "startLine": 6, "endLine": 6}
```

```rgbdns-fragment
{"id": "rgbdns-frag-bc885b4aa7d2", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "sourcePath": "src/lib.rs", "startLine": 7, "endLine": 7}
```
