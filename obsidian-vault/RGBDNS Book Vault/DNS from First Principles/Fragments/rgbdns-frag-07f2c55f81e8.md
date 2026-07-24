---
type: "code-fragment"
fragment_id: "rgbdns-frag-07f2c55f81e8"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "AXFR is a stream, not a giant datagram"
kind: "heading"
start_line: 558
end_line: 581
---

# AXFR is a stream, not a giant datagram

- Fragment ID: `rgbdns-frag-07f2c55f81e8`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 558-581
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-07f2c55f81e8", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-07f2c55f81e8: heading AXFR is a stream, not a giant datagram", "sourcePath": "docs/book/rgbdns.md", "startLine": 558, "endLine": 581}
```

## Excerpt

<span id="rgbdns-frag-07f2c55f81e8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-07f2c55f81e8: heading AXFR is a stream, not a giant datagram

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

`src/axfr.rs` provides both sides. `axfrdns` accepts TCP only and checks client
networks, loopback by default. It requires one AXFR question, obtains a
boundary-aware transfer from `Zone`, and frames bounded messages. `Zone::transfer`
excludes records beneath delegated child zones and wraps the result in the
apex SOA.

`axfr-get` generates a random transaction ID, validates response identity and
shape, collects records until the closing SOA, renders them in tinydns source
form, writes a temporary output, and atomically installs the completed file.
The temporary/final path pair prevents a failed transfer from replacing usable
data with a partial zone.

```
