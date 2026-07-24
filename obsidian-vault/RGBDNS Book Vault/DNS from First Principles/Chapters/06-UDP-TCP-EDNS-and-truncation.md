---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# UDP, TCP, EDNS, and truncation

## Why there are two transports

Classic DNS uses UDP for ordinary queries because one request and one response
need no connection setup. Traditional UDP DNS assumed a 512-byte message.
Larger answers set TC, telling the client to retry over TCP. TCP frames every
DNS message with a two-byte length.

Zone transfers use TCP. Modern responses—especially DNSSEC responses—often
need more than 512 bytes, so EDNS lets a client advertise a larger UDP receive
size through an OPT pseudo-record. Internet paths can still drop fragmented
UDP packets. A commonly conservative payload is 1232 bytes, large enough for
useful DNSSEC answers while fitting the IPv6 minimum MTU without fragmentation
under normal headers.

TCP is not merely an emergency protocol. Firewalls that assume DNS is
UDP-only break standards-compliant resolution.

## Truncation must preserve a valid message

Cutting the last bytes off an encoded message creates a malformed packet.
Correct truncation removes complete records, sets TC, updates section counts,
and re-encodes. A useful removal order discards nonessential additional data
before authority and answer data. OPT sometimes needs special treatment
because it carries the EDNS response.

`src/server.rs` calculates a response limit from the caller’s transport limit
and the client’s EDNS advertisement. It caps advertised UDP size, rejects
multiple OPT records, responds to unsupported EDNS versions, and constructs a
full typed response. If encoding exceeds the limit, `truncate` sets TC and
removes complete records in a defined order until the packet fits. The same
core response logic serves UDP and TCP without treating TCP as a giant UDP
datagram.

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
