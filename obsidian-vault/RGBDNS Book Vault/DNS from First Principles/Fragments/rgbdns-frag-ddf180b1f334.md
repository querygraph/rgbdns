---
type: "code-fragment"
fragment_id: "rgbdns-frag-ddf180b1f334"
source_path: "docs/blog/operationalizing-rgbdns/post.md"
code_note: "DNS from First Principles/Code/docs/blog/operationalizing-rgbdns/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "ANAME is an alias that can live at the apex"
kind: "heading"
start_line: 106
end_line: 147
---

# ANAME is an alias that can live at the apex

- Fragment ID: `rgbdns-frag-ddf180b1f334`
- Source file: [[DNS from First Principles/Code/docs/blog/operationalizing-rgbdns/post.md.source|docs/blog/operationalizing-rgbdns/post.md]]
- Lines: 106-147
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-ddf180b1f334", "codeNote": "DNS from First Principles/Code/docs/blog/operationalizing-rgbdns/post.md.source", "sourcePath": "docs/blog/operationalizing-rgbdns/post.md", "startLine": 106, "endLine": 147}
```

## Excerpt

<span id="rgbdns-frag-ddf180b1f334" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ddf180b1f334: heading ANAME is an alias that can live at the apex

````markdown
## ANAME is an alias that can live at the apex

A CNAME cannot normally coexist with the SOA, NS, and other records required
at a zone apex. Hosting platforms nevertheless want customers to point an
apex such as `fieldnotes.es` at a managed target whose addresses can change.
Providers solve this today under names such as CNAME flattening, ALIAS, or
ANAME.

rgbdns expresses the source intent directly in tinydns data syntax:

```text
Zfieldnotes.es:a.ns.cron.sh:hostmaster.fieldnotes.es:2026073103:16384:2048:1048576:2560:3600
&fieldnotes.es::a.ns.cron.sh:3600
&fieldnotes.es::b.ns.cron.sh:3600
Afieldnotes.es:publication.ghost.io:300
```

The uppercase `A` is an rgbdns ANAME directive: its owner is
`fieldnotes.es`, its target is `publication.ghost.io`, and 300 seconds is the
TTL ceiling. A normal resolver never needs to understand a private record.
When it asks for `A` or `AAAA`, each authoritative rgbdns server resolves the
target and returns ordinary address records at the ANAME owner. SOA, NS, MX,
TXT, and other apex data continue to coexist normally.

The subtle operational problem is transfer. Copying only today's synthesized
addresses to a secondary loses the target relationship and makes the primary's
temporary cache contents look like zone source. Copying an unknown private
type without negotiation risks collision with another implementation's
private encoding.

rgbdns therefore negotiates its experimental transfer extension explicitly.
An upgraded secondary sends the `RGA1` capability in EDNS option 65001. An
upgraded primary may then carry the ANAME source through private-use
TYPE65401, whose RDATA begins with `RGA1` and whose TTL carries the configured
ceiling. The secondary validates that envelope, reconstructs the uppercase
`A` directive, and resolves the target independently. Ordinary AXFR clients
receive an ordinary transfer without the private metadata.

The deployed values are experimental. The Internet-Draft requests assigned
ANAME and EDNS values as `TBD1` and `TBD2`; it does not pretend that 65401 and
65001 are permanent allocations.

````
