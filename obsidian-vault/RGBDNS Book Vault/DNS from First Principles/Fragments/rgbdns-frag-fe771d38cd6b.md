---
type: "code-fragment"
fragment_id: "rgbdns-frag-fe771d38cd6b"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "AXFR is a stream, not a giant datagram"
kind: "heading"
start_line: 686
end_line: 716
---

# AXFR is a stream, not a giant datagram

- Fragment ID: `rgbdns-frag-fe771d38cd6b`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 686-716
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-fe771d38cd6b", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-fe771d38cd6b: heading AXFR is a stream, not a giant datagram", "sourcePath": "docs/book/rgbdns.md", "startLine": 686, "endLine": 716}
```

## Excerpt

<span id="rgbdns-frag-fe771d38cd6b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-fe771d38cd6b: heading AXFR is a stream, not a giant datagram

```markdown
## AXFR is a stream, not a giant datagram

AXFR transfers a complete zone over TCP. A successful stream begins with the
zone’s SOA, contains the zone records, and ends with the SOA again. The records
may span many DNS messages. A client must continue until it sees the closing
SOA under the transfer rules; reading one response is insufficient.

Transfers reveal the zone contents and can consume resources, so authorities
normally restrict clients. TSIG is a common authentication mechanism in the
wider ecosystem, while IP allowlists are a simpler policy with weaker identity
properties.

`src/axfr.rs` provides both sides. The standalone `axfrdns` command accepts TCP
only and checks client networks, loopback by default. The packaged primary also
routes AXFR through `tinydns`'s existing TCP listener when `ALLOW_NETS` is set.
This is required when ordinary authoritative DNS and transfers must share one
address on port 53: two separate processes cannot own that TCP endpoint.

Both entry points require one AXFR question, obtain a boundary-aware transfer
from `Zone`, and frame bounded messages. `Zone::transfer` excludes records
beneath delegated child zones and wraps the result in the apex SOA. The
integrated listener applies its transfer allow-list only to AXFR; ordinary
DNS-over-TCP remains reachable by all clients allowed through the network
firewall.

`axfr-get` generates a random transaction ID, validates response identity and
shape, collects records until the closing SOA, renders them in tinydns source
form, writes a temporary output, and atomically installs the completed file.
The temporary/final path pair prevents a failed transfer from replacing usable
data with a partial zone.

```
