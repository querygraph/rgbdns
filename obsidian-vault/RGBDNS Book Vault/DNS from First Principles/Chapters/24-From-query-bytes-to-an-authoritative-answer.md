---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# From query bytes to an authoritative answer

[`server::respond`](../../src/server.rs) is the central authoritative pipeline.
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

The code separates mechanism from policy. [`transport.rs`](../../src/transport.rs)
knows UDP datagrams, TCP length prefixes, timeouts, persistent connections, and
a fixed worker bound. It knows nothing about zones. The handler knows DNS
policy but receives transport limits and client identity as ordinary
parameters. That separation lets specialized services reuse the network
machinery without pretending to be authoritative zones.

The original djbdns family achieved robustness partly through small processes.
rgbdns retains that decomposition while strengthening in-process boundaries.
The binaries under [`src/bin`](../../src/bin) are mostly adapters: environment,
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
{"id": "rgbdns-frag-4a9794b88127", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-4a9794b88127: fn compile", "sourcePath": "src/cdb.rs", "startLine": 12, "endLine": 52}
```

```rgbdns-fragment
{"id": "rgbdns-frag-83b463908a3c", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-83b463908a3c: fn load", "sourcePath": "src/cdb.rs", "startLine": 53, "endLine": 69}
```

```rgbdns-fragment
{"id": "rgbdns-frag-9e8e0d51389c", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-9e8e0d51389c: fn read_entries", "sourcePath": "src/cdb.rs", "startLine": 70, "endLine": 124}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4a71bdeba2ec", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-4a71bdeba2ec: fn decode_record", "sourcePath": "src/cdb.rs", "startLine": 125, "endLine": 178}
```

```rgbdns-fragment
{"id": "rgbdns-frag-a98232a8cdb0", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-a98232a8cdb0: fn encode_rdata", "sourcePath": "src/cdb.rs", "startLine": 179, "endLine": 237}
```

```rgbdns-fragment
{"id": "rgbdns-frag-1cfb12457767", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-1cfb12457767: fn decode_name", "sourcePath": "src/cdb.rs", "startLine": 238, "endLine": 264}
```

```rgbdns-fragment
{"id": "rgbdns-frag-908320134aee", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-908320134aee: fn le_u32", "sourcePath": "src/cdb.rs", "startLine": 265, "endLine": 269}
```

```rgbdns-fragment
{"id": "rgbdns-frag-489b39a43ae6", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-489b39a43ae6: mod tests", "sourcePath": "src/cdb.rs", "startLine": 270, "endLine": 275}
```

```rgbdns-fragment
{"id": "rgbdns-frag-563d0d5def13", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-563d0d5def13: fn exact_cdb_roundtrip_preserves_lookup_semantics", "sourcePath": "src/cdb.rs", "startLine": 276, "endLine": 324}
```

```rgbdns-fragment
{"id": "rgbdns-frag-af1b07391a27", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-af1b07391a27: fn rejects_truncated_database", "sourcePath": "src/cdb.rs", "startLine": 325, "endLine": 331}
```
