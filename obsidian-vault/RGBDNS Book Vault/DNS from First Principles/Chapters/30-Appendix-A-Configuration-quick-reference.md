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
| `DNSCACHEIP` | recursive endpoints used by client tools |
| `CACHESIZE` | bounded recursive response-cache capacity |
| `NSCACHESIZE` | bounded nameserver-cache entries |
| `RECURSION_LIMIT` | ordinary recursion depth |
| `NS_RECURSION_LIMIT` | nameserver-resolution recursion depth |
| `ROOT` | djbdns-compatible resolver configuration root |

Use the command’s `*-conf` generator as a starting point, then adapt the
foreground `run` contract to the chosen native supervisor.

## Generated Code Fragment Index

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-cfb0faf3b090", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-cfb0faf3b090: heading rgbdns", "sourcePath": "README.md", "startLine": 1, "endLine": 43}
```

```rgbdns-fragment
{"id": "rgbdns-frag-31920af81303", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-31920af81303: heading Book", "sourcePath": "README.md", "startLine": 44, "endLine": 63}
```

```rgbdns-fragment
{"id": "rgbdns-frag-855d7f20ae34", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-855d7f20ae34: heading Conformance and performance", "sourcePath": "README.md", "startLine": 64, "endLine": 75}
```

```rgbdns-fragment
{"id": "rgbdns-frag-d26839b00cc0", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-d26839b00cc0: mod axfr", "sourcePath": "src/lib.rs", "startLine": 3, "endLine": 3}
```

```rgbdns-fragment
{"id": "rgbdns-frag-c0387f24e3b2", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-c0387f24e3b2: mod cdb", "sourcePath": "src/lib.rs", "startLine": 4, "endLine": 4}
```

```rgbdns-fragment
{"id": "rgbdns-frag-ea9a942c2c24", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-ea9a942c2c24: mod client", "sourcePath": "src/lib.rs", "startLine": 5, "endLine": 5}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4ca6b4a66ffe", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-4ca6b4a66ffe: mod conf", "sourcePath": "src/lib.rs", "startLine": 6, "endLine": 6}
```

```rgbdns-fragment
{"id": "rgbdns-frag-47cdd9e7aa73", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-47cdd9e7aa73: mod dnscache_config", "sourcePath": "src/lib.rs", "startLine": 7, "endLine": 7}
```

```rgbdns-fragment
{"id": "rgbdns-frag-64be14e515dc", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-64be14e515dc: mod multilog", "sourcePath": "src/lib.rs", "startLine": 8, "endLine": 8}
```

```rgbdns-fragment
{"id": "rgbdns-frag-1198b492e8d3", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-1198b492e8d3: mod name", "sourcePath": "src/lib.rs", "startLine": 9, "endLine": 9}
```

```rgbdns-fragment
{"id": "rgbdns-frag-9a69adb381ac", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-9a69adb381ac: mod packet", "sourcePath": "src/lib.rs", "startLine": 10, "endLine": 10}
```

```rgbdns-fragment
{"id": "rgbdns-frag-cc4b83e1818c", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-cc4b83e1818c: mod pick", "sourcePath": "src/lib.rs", "startLine": 11, "endLine": 11}
```
