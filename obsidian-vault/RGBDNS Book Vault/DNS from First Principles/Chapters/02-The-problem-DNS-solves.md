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
