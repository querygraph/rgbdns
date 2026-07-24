---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Messages on the wire

## The twelve-byte header

A DNS message begins with a fixed twelve-byte header:

```text
0                   1                   2                   3
0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-------------------------------+-------------------------------+
|              ID               |            flags              |
+-------------------------------+-------------------------------+
|          question count       |          answer count         |
+-------------------------------+-------------------------------+
|         authority count       |         additional count      |
+-------------------------------+-------------------------------+
```

The transaction ID lets a client associate a response with a query. Important
flags include QR (query versus response), opcode, AA (authoritative answer), TC
(truncated), RD (recursion desired), RA (recursion available), and the
four-bit response code.

The four following sections contain questions, answers, authority records, and
additional records. A normal question carries a name, requested type, and
class. Resource-record sections add TTL, RDATA length, and RDATA.

All multibyte integers are network byte order. Every count and length comes
from an untrusted peer. A decoder must prove that bytes exist before reading
them, cap allocations, reject invalid labels and pointers, and finish with a
coherent message rather than a partially trusted structure.

## Name compression

Repeating full names would waste scarce datagram space. DNS permits a name
suffix to be replaced by a two-byte pointer whose high bits are `11` and whose
remaining bits are an offset earlier in the message.

Compression turns name decoding into graph traversal. A malicious packet can
contain a pointer loop, excessive indirection, or an offset outside the packet.
A safe decoder tracks visited offsets or imposes a strict jump bound, checks
every target, and separately enforces the expanded 255-octet name limit.

rgbdns’s `Reader` in `src/packet.rs` keeps all reads within a borrowed byte
slice. Name decoding validates pointer targets and bounds traversal. Record
decoding confines each RDATA parser to the declared RDLENGTH. EDNS option
iteration likewise checks the option header and value before advancing.

The `Writer` performs the reverse operation. Encoding is fallible: counts must
fit 16 bits, names and RDATA must fit their fields, and the result must remain
valid. This symmetry—decode into typed data, manipulate typed data, encode with
checks—is the packet layer’s central safety property.

## Errors are protocol results

Several results that sound similar are materially different:

- **NOERROR with answers**: the requested RRset exists.
- **NOERROR without answers**, often called NODATA: the name exists but the
  requested type does not.
- **NXDOMAIN**: the queried name does not exist.
- **SERVFAIL**: the server could not safely complete processing.
- **REFUSED**: policy forbids the operation.
- **FORMERR**: the message is malformed.
- **NOTIMP**: the requested opcode is unsupported.

Negative answers normally include the zone’s SOA so resolvers can cache the
negative result. Confusing NODATA with NXDOMAIN can suppress other valid types
at the same name.

rgbdns expresses authoritative lookup outcomes as `Lookup::Answer`,
`Referral`, `NoData`, `NxDomain`, and `Refused`. That internal sum type forces
the response builder to handle each protocol meaning explicitly.

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
