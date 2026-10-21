---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Appendix A: Configuration quick reference

Common daemon variables include:

| Variable | Meaning |
|---|---|
| `IP` | listen address |
| `PORT` | listen port |
| `DATA` | authoritative text or CDB path where supported |
| `ALLOW_NETS` | comma-separated client CIDRs for recursion or transfer |
| `DNSCACHEIP` | recursive endpoints used by client tools and ANAME flattening |
| `CACHESIZE` | bounded recursive response-cache capacity |
| `NSCACHESIZE` | bounded nameserver-cache entries |
| `RECURSION_LIMIT` | ordinary recursion depth |
| `NS_RECURSION_LIMIT` | nameserver-resolution recursion depth |
| `ROOT` | djbdns-compatible resolver configuration root |
| `DNSSEC_POLICY` | path to the one-line-per-zone `K`/`U` signing policy |

The authoritative DNSSEC utilities compose without a manager process:

```text
rgbsec-keygen zone keyfile
rgbsec-sign [data [data.signed]]
rgbsec-data [data [data.cdb]]
rgbsec-ds 'Kzone:keyfile:13:validity:refresh:skew'
rgbsec-check [data.cdb [dnssec-policy]]
```

Use the command’s `*-conf` generator as a starting point, then adapt the
foreground `run` contract to the chosen native supervisor.

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
