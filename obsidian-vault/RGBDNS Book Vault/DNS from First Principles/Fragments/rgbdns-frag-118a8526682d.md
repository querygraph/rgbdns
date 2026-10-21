---
type: "code-fragment"
fragment_id: "rgbdns-frag-118a8526682d"
source_path: "docs/conformance.md"
code_note: "DNS from First Principles/Code/docs/conformance.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Deliberate scope boundaries"
kind: "heading"
start_line: 53
end_line: 75
---

# Deliberate scope boundaries

- Fragment ID: `rgbdns-frag-118a8526682d`
- Source file: [[DNS from First Principles/Code/docs/conformance.md.source|docs/conformance.md]]
- Lines: 53-75
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-118a8526682d", "codeNote": "DNS from First Principles/Code/docs/conformance.md.source", "sourcePath": "docs/conformance.md", "startLine": 53, "endLine": 75}
```

## Excerpt

<span id="rgbdns-frag-118a8526682d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-118a8526682d: heading Deliberate scope boundaries

```markdown
## Deliberate scope boundaries

The following are not claimed as implemented by rgbdns 0.6.5:

- general-purpose dynamic UPDATE, NOTIFY, IXFR, DSO, DNS Cookies, or SIG(0);
- TSIG algorithms other than the ACME update profile's HMAC-SHA256;
- automated multi-key DNSSEC rollover, NSEC3, CDS, or CDNSKEY;
- DNS over TLS, HTTPS, or QUIC;
- a general RFC 1035 master-file parser (rgbdns uses tinydns source and CDB);
- recursive protocol internals supplied by Hickory beyond rgbdns's
  configuration, access-control, DNSSEC, and live-network tests.

Unknown EDNS options are parsed structurally and ignored as RFC 6891 requires.
Unknown ordinary RR types use opaque RDATA as RFC 3597 requires. An unsupported
opcode receives NOTIMP rather than being silently dropped.

The rgbdns ANAME AXFR extension is explicitly private and negotiated. Tests
prove that an opted-in rgbdns peer round-trips the target and TTL, a standard
AXFR request receives no private record, and malformed private payloads are
rejected before zone activation. ANAME resolver tests also prove that
concurrent misses are coalesced and that a failed target is briefly suppressed
instead of immediately starting another recursive chain.

```
