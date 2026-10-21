---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Zone transfer and secondary service

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

## Generated Code Fragment Index

Each card names an rgbdns fragment. In FirstPair Reader, Show code lists its numbered lines and Open file opens the collocated code note with those lines marked.

```rgbdns-fragment
{"id": "rgbdns-frag-ad1ed1e4cb84", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 15, "endLine": 15}
```

```rgbdns-fragment
{"id": "rgbdns-frag-169fc0dbbb4a", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 16, "endLine": 16}
```

```rgbdns-fragment
{"id": "rgbdns-frag-704dd44da912", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 17, "endLine": 17}
```

```rgbdns-fragment
{"id": "rgbdns-frag-2c15fff4d0a4", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 18, "endLine": 18}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f0f0f96758df", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 19, "endLine": 19}
```

```rgbdns-fragment
{"id": "rgbdns-frag-cafa2f19c1e6", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 20, "endLine": 20}
```

```rgbdns-fragment
{"id": "rgbdns-frag-8f7dc0cc0d16", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 21, "endLine": 22}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f2ef2cf7d6dd", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 23, "endLine": 30}
```

```rgbdns-fragment
{"id": "rgbdns-frag-d01b28bcef19", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 31, "endLine": 64}
```

```rgbdns-fragment
{"id": "rgbdns-frag-0b662b17240e", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 65, "endLine": 73}
```

```rgbdns-fragment
{"id": "rgbdns-frag-d76d0819e64d", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 74, "endLine": 154}
```

```rgbdns-fragment
{"id": "rgbdns-frag-6e33bc878cc9", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 177, "endLine": 198}
```
