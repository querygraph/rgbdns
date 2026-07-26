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
