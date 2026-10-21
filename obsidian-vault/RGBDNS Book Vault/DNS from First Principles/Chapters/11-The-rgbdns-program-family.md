---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# The rgbdns program family

## One suite, small purposes

rgbdns deliberately exposes separate commands:

| Command family | Purpose |
|---|---|
| `tinydns`, `tinydns-data`, `tinydns-get`, `tinydns-edit` | authoritative service and data maintenance |
| `dnscache` | validating recursive resolver and cache |
| `axfrdns`, `axfr-get` | zone transfer server and client |
| `rbldns`, `rbldns-data` | address-prefix blocklist DNS |
| `pickdns`, `pickdns-data` | location-aware address selection |
| `walldns` | synthetic address/reverse answers |
| `dnsq`, `dnsqr`, `dnsip*`, `dnsname`, `dnsmx`, `dnstxt` | queries and diagnostics |
| `dnsfilter`, `dnstrace`, `random-ip` | stream lookup, delegation tracing, testing |
| `rgbdns-log-report` | daily per-zone query and distinct-client aggregation |
| `*-conf` | service-directory generation |
| `setuidgid`, `multilog`, `tai64n`, `tai64nlocal` | process and logging support |

This composition makes privilege and failure boundaries visible. A compiler
can run with write access to data while the server runs read-only. A recursive
cache can be restarted without touching authority. Diagnostic clients reuse
the packet and client libraries rather than embedding daemon behavior.

## Specialized responders

`rbldns` treats the labels before a configured suffix as a numeric address,
finds the most-specific matching IPv4 prefix in a compiled database, and
returns configured A/TXT data. Parsing caps the number of numeric labels and
validates networks before compilation.

`pickdns` maps client prefixes to two-byte locations and selects address sets
for that location. It shuffles eligible addresses with operating-system
randomness. Location-aware answers are a policy feature; clients behind shared
resolvers may appear at the resolver’s address, a limitation operators must
understand.

`walldns` synthesizes narrowly defined forward and reverse answers without a
zone database. These specialized services run through `src/special.rs`, which
provides shared bounded UDP/TCP serving and passes the peer address to the
handler.

The lesson is architectural: once parsing, transport, names, and record models
are sound, unusual DNS policies can be small pure response functions rather
than new monolithic servers.

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
