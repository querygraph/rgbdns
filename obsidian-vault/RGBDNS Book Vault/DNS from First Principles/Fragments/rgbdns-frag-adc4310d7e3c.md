---
type: "code-fragment"
fragment_id: "rgbdns-frag-adc4310d7e3c"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Conformance as an executable specification"
kind: "heading"
start_line: 1268
end_line: 1323
---

# Conformance as an executable specification

- Fragment ID: `rgbdns-frag-adc4310d7e3c`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1268-1323
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-adc4310d7e3c", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-adc4310d7e3c: heading Conformance as an executable specification", "sourcePath": "docs/book/rgbdns.md", "startLine": 1268, "endLine": 1323}
```

## Excerpt

<span id="rgbdns-frag-adc4310d7e3c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-adc4310d7e3c: heading Conformance as an executable specification

```markdown
## Conformance as an executable specification

The conformance suite turns protocol prose into named, reviewable cases. Its
scope is the DNS surface rgbdns implements; it does not imply support for every
extension ever assigned by IANA. The principal coverage is:

| Standard | Behavior exercised |
|---|---|
| RFC 1035 | header identity, flags, names, compression, typed RDATA, UDP and TCP results |
| RFC 2181 | in-bailiwick glue, coherent RRset TTLs, duplicate suppression, CNAME exclusivity |
| RFC 2308 | NXDOMAIN versus NODATA, authoritative SOA, negative-cache TTL |
| RFC 3597 | unknown QTYPE behavior and lossless opaque RDATA |
| RFC 4343 | case-insensitive name identity with query-case preservation |
| RFC 4592 | closest-encloser wildcard synthesis and empty non-terminals |
| RFC 5936 | AXFR framing, identity, flags, SOA bookends, and zone boundaries |
| RFC 6891 | one root-owned OPT, payload negotiation, DO, BADVERS, and unknown options |
| RFC 7766 | TCP framing, connection reuse, pipelining, and full-size responses |
| RFC 8906 | the authoritative-server matrix for unknown types, opcodes, flags, and EDNS fields |
| RFC 9619 | exactly one question in a standard query |

This is more useful than a single “RFC compliant” label. A test name identifies
the rule, a packet fixture demonstrates it, and a failure points to a specific
semantic regression.

RFC 8906 is especially valuable because it tests how a server behaves at the
edges of what it understands. An unknown ordinary type is not a protocol
error: the answer depends on whether the owner name exists. An unknown opcode
is different. Because that opcode may define a body layout unlike QUERY, the
server must produce NOTIMP from the header without first interpreting the body
as an ordinary question. Unknown EDNS options are structurally validated and
then ignored. An unsupported EDNS version produces BADVERS while retaining an
OPT response.

The independent `drill` integration test supplies another boundary. It launches
the real `tinydns` binary and asks the ldns client to make UDP, TCP, EDNS,
mixed-case, and unknown-type queries. This catches accidental agreement
between rgbdns's own encoder and decoder: the request and response cross an
implementation boundary.

The complete focused matrix is:

```sh
cargo test --test rfc_conformance
cargo test --test wire_security
cargo test --test packet_properties
cargo test --test drill_interop
```

The generated suite exercises forty thousand cases per complete run. It feeds
arbitrary bytes to the decoder, reparses every accepted packet, generates
structured messages for semantic round trips, and changes ASCII letter case
without changing DNS name identity. A separate truncation corpus tries every
prefix of a valid structured packet. These properties do not prove the absence
of all parser defects, but they explore combinations that hand-written examples
rarely anticipate.

```
