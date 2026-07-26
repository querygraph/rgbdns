---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Resource records: typed facts with lifetimes

## The common envelope

Every resource record has:

- an owner name;
- a numeric type;
- a class, almost always Internet class `IN`;
- a time to live, or TTL;
- type-specific data called RDATA.

The TTL is a lease offered to caches. If an authoritative server returns a TTL
of 300 seconds, a cache may reuse that answer for at most five minutes before
refreshing it. The TTL does not schedule a change and does not guarantee that
every cache holds the answer for the full interval. It establishes an upper
bound.

Changing a record and then lowering its TTL is too late for clients that
already cached the older, longer lease. Planned migrations lower the TTL at
least one old-TTL interval before the change, wait, make the change, and later
raise it.

## Core types

**A** maps an owner to an IPv4 address. **AAAA** maps it to an IPv6 address.
Several records at one owner form an address RRset; DNS does not promise that
clients use them in listed order.

**NS** names an authoritative server for a zone or delegation.

**SOA**, the start of authority, identifies the zone and carries operational
parameters: primary server, responsible mailbox, serial, refresh, retry,
expire, and negative-cache values. A secondary compares serial numbers to
decide whether a transfer is needed. Serial arithmetic wraps in a defined
32-bit space, so blindly treating it as an ordinary integer can be wrong near
the boundary.

**CNAME** says that its owner is an alias of another name. Except for DNSSEC
and narrowly specified metadata, an owner with CNAME should not also hold
unrelated data. A resolver follows the chain while defending against loops and
excessive depth.

**ANAME** is not a standard DNS record type in rgbdns. It is authoritative
server configuration that copies the address meaning of another name onto an
owner. This distinction matters: clients receive ordinary A or AAAA records,
not an ANAME record. ANAME can therefore provide address aliasing at a zone
apex without violating CNAME exclusivity.

**MX** names a mail exchanger and gives it a preference. Lower numbers are
preferred. The target is a name, not an address.

**PTR** provides a name-valued reverse mapping. IPv4 reverse names live below
`in-addr.arpa.` with octets reversed. IPv6 reverse names live below
`ip6.arpa.` with hexadecimal nibbles reversed.

**TXT** carries one or more length-delimited byte strings. Presentation formats
often make it look like one free-form string, but the wire format retains
segments.

**SRV** names a service endpoint with priority, weight, port, and target.

**CAA** constrains which certification authorities may issue certificates for
a domain.

**OPT** is not ordinary zone data. It is a pseudo-record used by EDNS to
negotiate UDP payload size and carry extension flags and options.

rgbdns models these forms with `RecordType`, `Record`, and the `RData` enum in
`src/packet.rs`. Known structured types receive structured variants. Unknown
types can remain opaque where the format permits, preserving extensibility
without confusing untrusted lengths with trusted objects.

## Additional data is an optimization

If an answer contains MX, NS, or SRV targets, the server may include associated
A and AAAA records in the additional section. This can save queries. It does
not change which RRset directly answers the question, and a resolver must apply
the correct credibility rules rather than trusting unrelated additional data.

The rgbdns authoritative response path collects target names from those record
types and adds locally available addresses. It de-duplicates targets before
lookup and preserves the distinction between answers and helpful additionals.

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
