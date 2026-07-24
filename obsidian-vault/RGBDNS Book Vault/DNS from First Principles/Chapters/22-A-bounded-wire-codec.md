---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# A bounded wire codec

DNS packets are attacker-controlled binary graphs: compression pointers can
jump backward, names can share suffixes, section counts can lie, and RDATA
lengths can disagree with actual bytes. [`packet.rs`](../../src/packet.rs)
contains the codec and deliberately keeps the reader state small:

```rust
struct Reader<'a> {
    b: &'a [u8],
    p: usize,
    name_offsets: Vec<bool>,
}
```

The lifetime on `b` prevents the reader from outliving its input. Every scalar
read uses slice bounds checks. Name decoding separately limits pointer hops,
requires pointers to move backward, and records valid prior name boundaries.
The last rule is stricter than merely checking that a pointer lands inside the
packet: an interior byte can accidentally look like a valid label.

Decoded records become an `RData` variant. Unknown types are not discarded;
they become opaque bytes paired with their numeric `RecordType`. This is the
extension-safe behavior required by modern DNS. It also means an encoder can
round-trip data it does not understand.

The writer uses compression, but optimization remains subordinate to a valid
message. A last-owner cache handles repeated owners cheaply while suffix
sharing reduces wire size across related names. The July 2026 benchmark shows
the trade: a 64-record answer fell from 2,147 to 1,059 bytes, while compression
made encoding slower than the uncompressed baseline. On DNS, fewer datagrams
and less amplification surface can be worth several microseconds of local CPU.

Truncation uses a bounded search for the largest response that fits instead of
repeatedly rebuilding one record at a time. The result preserves complete
RRsets and required EDNS state. Performance work is therefore expressed as an
algorithmic improvement behind the same `Message::encode` contract.

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
