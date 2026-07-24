---
type: "code-fragment"
fragment_id: "rgbdns-frag-fe7f8140d5e8"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Core types"
kind: "heading"
start_line: 186
end_line: 229
---

# Core types

- Fragment ID: `rgbdns-frag-fe7f8140d5e8`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 186-229
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-fe7f8140d5e8", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-fe7f8140d5e8: heading Core types", "sourcePath": "docs/book/rgbdns.md", "startLine": 186, "endLine": 229}
```

## Excerpt

<span id="rgbdns-frag-fe7f8140d5e8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-fe7f8140d5e8: heading Core types

```markdown
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

```
