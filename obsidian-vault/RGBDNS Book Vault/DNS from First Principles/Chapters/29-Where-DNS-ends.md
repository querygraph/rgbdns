---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Where DNS ends

DNS establishes named, cacheable facts and, with DNSSEC, their authenticated
origin. It does not prove that the address belongs to the application a user
intended, encrypt the subsequent connection, guarantee freshness inside the
TTL window, or choose a healthy endpoint. TLS identity, application discovery,
load balancing, routing, and monitoring build on DNS but remain separate
systems.

That boundary is the best final replacement for the phone-book metaphor. DNS
is a delegated publication and discovery protocol. Its tree assigns authority;
its records carry typed statements; its TTLs make caching explicit; its packet
format makes efficient exchange possible; recursion joins many authorities
into one answer; DNSSEC authenticates the chain; and supervision keeps the
implementing processes available without becoming part of the protocol.

rgbdns expresses those ideas as small programs over shared, validated Rust
types. Understanding the protocol makes the program family unsurprising.
Reading the program family, in turn, shows how the abstract DNS model becomes
bounded packets, immutable databases, iterative queries, atomic files, and
foreground processes.

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
