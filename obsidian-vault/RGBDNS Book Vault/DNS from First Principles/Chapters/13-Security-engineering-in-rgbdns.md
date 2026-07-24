---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Security engineering in rgbdns

## The packet is hostile

DNS combines nearly every parser hazard: nested lengths, compression pointers,
variable counts, binary strings, recursive relationships, and network-facing
availability requirements. “Written in Rust” removes broad classes of memory
corruption, but it does not automatically prevent allocation bombs, infinite
loops, CPU amplification, path races, policy errors, or accepting incoherent
messages.

rgbdns therefore uses several layers:

- `#![forbid(unsafe_code)]` for the library;
- explicit bounds before every wire read;
- validated `Name`, `Message`, and `RData` objects;
- limits on compression traversal, aliases, records, files, configuration
  lists, recursion, transfers, and cache sizes;
- cryptographic operating-system randomness for query IDs and selection;
- complete-record truncation;
- loopback-only defaults for recursion and transfer;
- atomic replacement for compiled databases and fetched zones;
- no shell interpolation when replacing a process.

Property tests in `tests/packet_properties.rs` feed arbitrary bytes to the
decoder and exercise encode/decode invariants. Golden CDB fixtures compare
compiled output with the expected djbdns layout. Network tests cross real UDP
and TCP boundaries. Compatibility tests are valuable here because a parser can
be safe yet subtly wrong, or compatible yet unsafe.

## Least privilege and filesystem boundaries

The `*-conf` commands generate service directories whose run scripts execute
the daemon under a selected account. rgbdns’s `setuidgid` resolves the user and
group, initializes supplementary groups, drops GID and UID, verifies the
result, and directly replaces itself with the target program. Direct
replacement preserves signals and exit status and avoids an extra shell-owned
process.

Generated paths are shell-quoted and support binaries by absolute path.
Configuration writers reject unsafe existing file types and apply intentional
modes. CDB and AXFR update workflows install only complete outputs.

Privilege dropping is not a substitute for a restricted service account,
read-only data, network policy, or supervisor hardening. It is one layer in a
deployment.

## Generated Code Fragment Index

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-b92db435b523", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-b92db435b523: enum RecordType", "sourcePath": "src/packet.rs", "startLine": 8, "endLine": 27}
```

```rgbdns-fragment
{"id": "rgbdns-frag-19e8e1a6ba89", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-19e8e1a6ba89: impl RecordType", "sourcePath": "src/packet.rs", "startLine": 28, "endLine": 28}
```

```rgbdns-fragment
{"id": "rgbdns-frag-e314247d27ae", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-e314247d27ae: fn code", "sourcePath": "src/packet.rs", "startLine": 29, "endLine": 50}
```

```rgbdns-fragment
{"id": "rgbdns-frag-9f94a1e1d010", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-9f94a1e1d010: fn from_code", "sourcePath": "src/packet.rs", "startLine": 51, "endLine": 73}
```

```rgbdns-fragment
{"id": "rgbdns-frag-e35eb9c748af", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-e35eb9c748af: impl std", "sourcePath": "src/packet.rs", "startLine": 74, "endLine": 74}
```

```rgbdns-fragment
{"id": "rgbdns-frag-557f873dfbf4", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-557f873dfbf4: type Err", "sourcePath": "src/packet.rs", "startLine": 75, "endLine": 75}
```

```rgbdns-fragment
{"id": "rgbdns-frag-5f4505fe3381", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-5f4505fe3381: fn from_str", "sourcePath": "src/packet.rs", "startLine": 76, "endLine": 117}
```

```rgbdns-fragment
{"id": "rgbdns-frag-1f43add77363", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-1f43add77363: struct Question", "sourcePath": "src/packet.rs", "startLine": 118, "endLine": 123}
```

```rgbdns-fragment
{"id": "rgbdns-frag-8a53d16528c4", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-8a53d16528c4: struct Record", "sourcePath": "src/packet.rs", "startLine": 124, "endLine": 128}
```

```rgbdns-fragment
{"id": "rgbdns-frag-3474979ebcab", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-3474979ebcab: impl Record", "sourcePath": "src/packet.rs", "startLine": 129, "endLine": 129}
```

```rgbdns-fragment
{"id": "rgbdns-frag-1a1d0297797e", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-1a1d0297797e: fn rr_type", "sourcePath": "src/packet.rs", "startLine": 130, "endLine": 135}
```

```rgbdns-fragment
{"id": "rgbdns-frag-edc236285edc", "codeNote": "DNS from First Principles/Code/src/packet.rs.source", "heading": "rgbdns-frag-edc236285edc: enum RData", "sourcePath": "src/packet.rs", "startLine": 136, "endLine": 170}
```
