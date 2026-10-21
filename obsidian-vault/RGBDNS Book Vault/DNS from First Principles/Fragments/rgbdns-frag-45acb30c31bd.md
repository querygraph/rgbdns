---
type: "code-fragment"
fragment_id: "rgbdns-frag-45acb30c31bd"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "One request, one portable log record"
kind: "heading"
start_line: 1031
end_line: 1063
---

# One request, one portable log record

- Fragment ID: `rgbdns-frag-45acb30c31bd`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1031-1063
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-45acb30c31bd", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 1031, "endLine": 1063}
```

## Excerpt

<span id="rgbdns-frag-45acb30c31bd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-45acb30c31bd: heading One request, one portable log record

````markdown
## One request, one portable log record

The original tinydns wrote one compact record for every request. rgbdns keeps
that operational contract. An IPv4 record has this shape:

```text
7f000001:e214:0018 + 0001 fieldnotes.es
```

The client address, source port, DNS ID, and query type are hexadecimal. The
result marker distinguishes an attempted answer (`+`), refused authority or
AXFR (`-`), an unimplemented request (`I`), an unsupported class (`C`), and a
malformed request (`/`). IPv6 uses the same format with a 32-hex-digit address.
Query names are escaped so packet data cannot inject additional log lines.

The record goes to stderr without a timestamp. The packaged systemd service
therefore sends it to journald, while a daemontools service can send the exact
same stream through `multilog t` for TAI64N timestamps and file rotation.
`QUERY_LOG=1` is the default. `QUERY_LOG=0` is an explicit opt-out for an
installation whose traffic, retention, or privacy policy forbids full query
logging.

`rgbdns-log-report` turns a bounded daily slice of that stream into an
operational summary. It maps each accepted query name to the longest matching
configured authoritative zone, so `www.wishful.ly` contributes to
`wishful.ly` rather than becoming a separate domain. For each zone it reports
the total query count and the number of distinct client addresses, then sorts
by total descending. A packaged opt-in timer reads the preceding local day
from journald and submits the text through a sendmail-compatible transport.
Distinct addresses usually identify recursive resolvers, not individual
people, and DNS queries are not HTTP pageviews. The timer belongs on one
authority only when duplicate reports are unwanted.

````
