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
rgbdns 0.6.3 and book version 0.4.0 as built on 2026-08-21.

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
