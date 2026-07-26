---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Preface {-}

DNS is often introduced as “the Internet’s phone book.” That metaphor is
useful for about a minute. A phone book is one database, published in editions,
mapping people to telephone numbers. DNS is a distributed protocol for finding
typed, time-limited statements in a delegated tree. It has many writers, many
readers, caches between them, multiple transports, and rules for proving both
presence and absence. Names can point to addresses, but they can also identify
mail exchangers, service endpoints, authoritative servers, cryptographic keys,
and arbitrary text.

This book develops DNS from those underlying problems. The first half builds a
mental model independent of any implementation. The second half walks through
rgbdns, a memory-safe Rust reimplementation of the djbdns suite. The aim is not
only to explain what each program does, but why its boundaries look the way
they do: immutable compiled data for authority, a separate recursive cache,
small diagnostic clients, foreground daemons, and stream-oriented logging.

The code is the final authority for rgbdns behavior. This book describes
version 0.1.0 as built on 2026-07-23.

## Generated Code Fragment Index

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-5a01ae2d3ddb", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-5a01ae2d3ddb: heading rgbdns", "sourcePath": "README.md", "startLine": 1, "endLine": 56}
```

```rgbdns-fragment
{"id": "rgbdns-frag-c24b9da16705", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-c24b9da16705: heading Debian and systemd", "sourcePath": "README.md", "startLine": 57, "endLine": 74}
```

```rgbdns-fragment
{"id": "rgbdns-frag-003170c20cd5", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-003170c20cd5: heading Book", "sourcePath": "README.md", "startLine": 75, "endLine": 94}
```

```rgbdns-fragment
{"id": "rgbdns-frag-851308e1cfeb", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-851308e1cfeb: heading Conformance and performance", "sourcePath": "README.md", "startLine": 95, "endLine": 106}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f52ccf723277", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-f52ccf723277: mod aname", "sourcePath": "src/lib.rs", "startLine": 3, "endLine": 3}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4a817a7124e1", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-4a817a7124e1: mod axfr", "sourcePath": "src/lib.rs", "startLine": 4, "endLine": 4}
```

```rgbdns-fragment
{"id": "rgbdns-frag-ef82c203a6e1", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-ef82c203a6e1: mod cdb", "sourcePath": "src/lib.rs", "startLine": 5, "endLine": 5}
```

```rgbdns-fragment
{"id": "rgbdns-frag-ac93886065b4", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-ac93886065b4: mod client", "sourcePath": "src/lib.rs", "startLine": 6, "endLine": 6}
```

```rgbdns-fragment
{"id": "rgbdns-frag-558fa31b05c5", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-558fa31b05c5: mod conf", "sourcePath": "src/lib.rs", "startLine": 7, "endLine": 7}
```

```rgbdns-fragment
{"id": "rgbdns-frag-fa79f1453710", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-fa79f1453710: mod dnscache_config", "sourcePath": "src/lib.rs", "startLine": 8, "endLine": 8}
```

```rgbdns-fragment
{"id": "rgbdns-frag-060fb35dda55", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-060fb35dda55: mod multilog", "sourcePath": "src/lib.rs", "startLine": 9, "endLine": 9}
```

```rgbdns-fragment
{"id": "rgbdns-frag-9b91c16392f6", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-9b91c16392f6: mod name", "sourcePath": "src/lib.rs", "startLine": 10, "endLine": 10}
```
