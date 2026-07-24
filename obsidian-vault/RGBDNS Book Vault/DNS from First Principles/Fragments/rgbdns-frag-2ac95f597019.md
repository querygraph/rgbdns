---
type: "code-fragment"
fragment_id: "rgbdns-frag-2ac95f597019"
source_path: "docs/conformance.md"
code_note: "DNS from First Principles/Code/docs/conformance.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "DNS conformance and security matrix"
kind: "heading"
start_line: 1
end_line: 28
---

# DNS conformance and security matrix

- Fragment ID: `rgbdns-frag-2ac95f597019`
- Source file: [[DNS from First Principles/Code/docs/conformance.md.source|docs/conformance.md]]
- Lines: 1-28
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-2ac95f597019", "codeNote": "DNS from First Principles/Code/docs/conformance.md.source", "heading": "rgbdns-frag-2ac95f597019: heading DNS conformance and security matrix", "sourcePath": "docs/conformance.md", "startLine": 1, "endLine": 28}
```

## Excerpt

<span id="rgbdns-frag-2ac95f597019" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2ac95f597019: heading DNS conformance and security matrix

```markdown
# DNS conformance and security matrix

This matrix makes rgbdns's test claims traceable. It covers the protocol surface
implemented by rgbdns; it does not claim that every DNS extension is
implemented. RFC keywords and section numbers refer to the linked canonical
RFC Editor copies.

The executable RFC cases live in `tests/rfc_conformance.rs`, hostile wire cases
in `tests/wire_security.rs`, generated cases in
`tests/packet_properties.rs`, live transport cases beside their implementations,
and independent ldns interoperability in `tests/drill_interop.rs`.

| Specification | Requirement exercised | Evidence |
|---|---|---|
| [RFC 1034](https://www.rfc-editor.org/rfc/rfc1034.html) / [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035.html) | Header identity and flags, names and compression, typed RDATA, referrals, CNAMEs, UDP and TCP, FORMERR/NOTIMP/REFUSED | `rfc1035_*`, packet unit tests, live server tests |
| [RFC 2181](https://www.rfc-editor.org/rfc/rfc2181.html) | In-bailiwick referral glue and coherent RRsets | `rfc2181_*`, server referral tests |
| [RFC 2308](https://www.rfc-editor.org/rfc/rfc2308.html) | NXDOMAIN versus NODATA, authoritative SOA, negative TTL minimum | `rfc2308_*` |
| [RFC 3597](https://www.rfc-editor.org/rfc/rfc3597.html) | Unknown QTYPE behavior and lossless opaque RDATA | `rfc3597_*`, `rfc8906_unknown_types_*` |
| [RFC 4033–4035](https://www.rfc-editor.org/rfc/rfc4035.html) | Validating recursion: secure answers get AD; bogus answers get SERVFAIL | ignored live test `dnscache_network` |
| [RFC 4343](https://www.rfc-editor.org/rfc/rfc4343.html) | ASCII-case-insensitive identity with query-case preservation | `rfc4343_*`, generated case properties |
| [RFC 4592](https://www.rfc-editor.org/rfc/rfc4592.html) | Closest encloser, empty non-terminals, wildcard blocking and synthesis | `rfc4592_*`, zone wildcard tests |
| [RFC 5936](https://www.rfc-editor.org/rfc/rfc5936.html) | AXFR framing, question and flag validation, SOA bookends, zone boundaries | AXFR unit and live-transfer tests |
| [RFC 6891](https://www.rfc-editor.org/rfc/rfc6891.html) | One root-owned OPT in additional, payload limit, BADVERS, DO, ignored unknown flags/options, OPT retained on truncation | `rfc6891_*`, `rfc8906_badvers_*` |
| [RFC 7766](https://www.rfc-editor.org/rfc/rfc7766.html) | TCP support, framing, connection reuse, pipelined queries, no UDP-size limit on TCP | live server TCP tests |
| [RFC 8482](https://www.rfc-editor.org/rfc/rfc8482.html) | Full ANY answers remain a permitted server policy | zone `RecordType::Any` tests |
| [RFC 8906 / BCP 231](https://www.rfc-editor.org/rfc/rfc8906.html) | Responses to unknown types, opcodes, flags, EDNS versions, flags and options | Section 8-style matrix in `rfc_conformance` |
| [RFC 9619](https://www.rfc-editor.org/rfc/rfc9619.html) | Standard queries have exactly one question; malformed counts get FORMERR | `rfc9619_*` |

```
