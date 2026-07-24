---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# The problem DNS solves

## Identity is not location

A network delivers packets to addresses. Humans and applications want stable
identities. Those two things should not be fused.

Suppose a service is reached at `192.0.2.8`. If that address is embedded in
every configuration, moving the service requires changing every client. A name
such as `api.example` introduces indirection:

```text
application → api.example → 192.0.2.8 → packets
```

Indirection has a cost: another system must answer the middle question. Its
benefit is that the service owner can change the answer without changing the
application. DNS is the globally deployed mechanism for this indirection.

The mapping is not a function from one name to one address. One name may have
several addresses. The answers may differ by client location. A mail domain
may name several mail exchangers with preferences. A service may delegate a
subtree to another organization. The useful abstraction is therefore:

```text
(owner name, record type, class) → a set of resource records
```

The owner and type together select an RRset. “RRset” means all resource records
with the same owner, type, and class. Implementations should normally treat the
set as a unit because caches and DNSSEC signatures do.

## Requirements that pull in different directions

A global naming system must satisfy conflicting demands:

- It must scale without one central database receiving every query.
- Different organizations must control different parts of the namespace.
- Changes must propagate, but cached answers are essential for performance.
- Replies should usually fit in one datagram, but some answers are large.
- Old implementations must coexist with protocol extensions.
- A client needs to distinguish “no such name” from “that name has no record
  of this type.”
- Operators need a way to transfer complete zones and to diagnose individual
  exchanges.

DNS answers these demands with hierarchy, delegation, caching lifetimes,
compact binary messages, UDP plus TCP, explicit result codes, and typed
records. Many operational surprises are direct consequences of those design
choices rather than random quirks.

## Roles, not just “DNS servers”

The phrase “DNS server” hides several jobs.

An **authoritative server** publishes data for zones it controls. It answers
from configured facts and does not chase referrals on behalf of a client.

A **recursive resolver** accepts a question from a stub client, follows the
delegation chain, validates and caches what it learns, and returns a final
answer.

A **stub resolver** is the client-side library or program that sends a
recursive query to a configured resolver.

A **forwarder** sends selected questions to another resolver rather than
performing iteration itself.

Keeping these roles distinct is both conceptual hygiene and a security
boundary. An authoritative daemon does not need a large mutable Internet-fed
cache. A recursive resolver does not need the private machinery used to edit a
zone. rgbdns follows the djbdns design and runs authority and recursion as
different programs.

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
