---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# From query bytes to an authoritative answer

[`server::respond`](https://github.com/querygraph/rgbdns/blob/master/src/server.rs) is the central authoritative pipeline.
Its shape is intentionally linear:

1. Reject an unknown opcode from the header without misparsing its body as a
   standard query.
2. Decode the packet, mapping malformed standard queries to `FORMERR`.
3. Enforce one question and valid OPT placement.
4. Derive the UDP response limit from EDNS and the transport ceiling.
5. Ask `Zone` for a typed `Lookup`.
6. Expand bounded CNAME chains and add relevant target addresses.
7. Normalize RRset TTLs and remove duplicates.
8. Encode or truncate the response.

The code separates mechanism from policy. [`transport.rs`](https://github.com/querygraph/rgbdns/blob/master/src/transport.rs)
knows UDP datagrams, TCP length prefixes, timeouts, persistent connections, and
a fixed worker bound. It knows nothing about zones. The handler knows DNS
policy but receives transport limits and client identity as ordinary
parameters. That separation lets specialized services reuse the network
machinery without pretending to be authoritative zones.

The original djbdns family achieved robustness partly through small processes.
rgbdns retains that decomposition while strengthening in-process boundaries.
The binaries under [`src/bin`](https://github.com/querygraph/rgbdns/tree/master/src/bin) are mostly adapters: environment,
configuration, a library call, and the djbdns-compatible fatal exit convention.
Small executables remain independently supervisable, but common logic is
testable as ordinary Rust functions.

## Generated Code Fragment Index

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-e5fff4b8cb2b", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-e5fff4b8cb2b: const HEADER_LEN", "sourcePath": "src/cdb.rs", "startLine": 9, "endLine": 9}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4907bb687e44", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-4907bb687e44: const MAX_DATABASE_SIZE", "sourcePath": "src/cdb.rs", "startLine": 10, "endLine": 11}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f318af8bdeaa", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-f318af8bdeaa: fn compile", "sourcePath": "src/cdb.rs", "startLine": 12, "endLine": 61}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f67eebb3c015", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-f67eebb3c015: fn load", "sourcePath": "src/cdb.rs", "startLine": 62, "endLine": 101}
```

```rgbdns-fragment
{"id": "rgbdns-frag-916ec1cbc28e", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-916ec1cbc28e: fn read_entries", "sourcePath": "src/cdb.rs", "startLine": 102, "endLine": 156}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f9047bc1a1a2", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-f9047bc1a1a2: fn decode_record", "sourcePath": "src/cdb.rs", "startLine": 157, "endLine": 210}
```

```rgbdns-fragment
{"id": "rgbdns-frag-bf06479bb119", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-bf06479bb119: fn encode_rdata", "sourcePath": "src/cdb.rs", "startLine": 211, "endLine": 269}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f2d13363a376", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-f2d13363a376: fn decode_name", "sourcePath": "src/cdb.rs", "startLine": 270, "endLine": 296}
```

```rgbdns-fragment
{"id": "rgbdns-frag-ca380a5004ce", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-ca380a5004ce: fn le_u32", "sourcePath": "src/cdb.rs", "startLine": 297, "endLine": 301}
```

```rgbdns-fragment
{"id": "rgbdns-frag-9cc58af3bb02", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-9cc58af3bb02: mod tests", "sourcePath": "src/cdb.rs", "startLine": 302, "endLine": 307}
```

```rgbdns-fragment
{"id": "rgbdns-frag-d600fc524f2b", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-d600fc524f2b: fn exact_cdb_roundtrip_preserves_lookup_semantics", "sourcePath": "src/cdb.rs", "startLine": 308, "endLine": 364}
```

```rgbdns-fragment
{"id": "rgbdns-frag-428b1ce3e4be", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-428b1ce3e4be: fn rejects_truncated_database", "sourcePath": "src/cdb.rs", "startLine": 365, "endLine": 371}
```
