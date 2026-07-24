---
type: "code-fragment"
fragment_id: "rgbdns-frag-6b9de5d06379"
source_path: "docs/conformance.md"
code_note: "DNS from First Principles/Code/docs/conformance.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Deliberate scope boundaries"
kind: "heading"
start_line: 51
end_line: 65
---

# Deliberate scope boundaries

- Fragment ID: `rgbdns-frag-6b9de5d06379`
- Source file: [[DNS from First Principles/Code/docs/conformance.md.source|docs/conformance.md]]
- Lines: 51-65
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-6b9de5d06379", "codeNote": "DNS from First Principles/Code/docs/conformance.md.source", "heading": "rgbdns-frag-6b9de5d06379: heading Deliberate scope boundaries", "sourcePath": "docs/conformance.md", "startLine": 51, "endLine": 65}
```

## Excerpt

<span id="rgbdns-frag-6b9de5d06379" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6b9de5d06379: heading Deliberate scope boundaries

```markdown
## Deliberate scope boundaries

The following are not claimed as implemented by rgbdns 0.1.1:

- dynamic UPDATE, NOTIFY, IXFR, DSO, DNS Cookies, or TSIG/SIG(0);
- authoritative DNSSEC signing and denial-of-existence generation;
- DNS over TLS, HTTPS, or QUIC;
- a general RFC 1035 master-file parser (rgbdns uses tinydns source and CDB);
- recursive protocol internals supplied by Hickory beyond rgbdns's
  configuration, access-control, DNSSEC, and live-network tests.

Unknown EDNS options are parsed structurally and ignored as RFC 6891 requires.
Unknown ordinary RR types use opaque RDATA as RFC 3597 requires. An unsupported
opcode receives NOTIMP rather than being silently dropped.

```
