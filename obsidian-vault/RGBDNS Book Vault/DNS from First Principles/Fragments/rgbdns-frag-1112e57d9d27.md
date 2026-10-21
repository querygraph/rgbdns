---
type: "code-fragment"
fragment_id: "rgbdns-frag-1112e57d9d27"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "One suite, small purposes"
kind: "heading"
start_line: 1066
end_line: 1088
---

# One suite, small purposes

- Fragment ID: `rgbdns-frag-1112e57d9d27`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1066-1088
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-1112e57d9d27", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 1066, "endLine": 1088}
```

## Excerpt

<span id="rgbdns-frag-1112e57d9d27" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1112e57d9d27: heading One suite, small purposes

```markdown
## One suite, small purposes

rgbdns deliberately exposes separate commands:

| Command family | Purpose |
|---|---|
| `tinydns`, `tinydns-data`, `tinydns-get`, `tinydns-edit` | authoritative service and data maintenance |
| `dnscache` | validating recursive resolver and cache |
| `axfrdns`, `axfr-get` | zone transfer server and client |
| `rbldns`, `rbldns-data` | address-prefix blocklist DNS |
| `pickdns`, `pickdns-data` | location-aware address selection |
| `walldns` | synthetic address/reverse answers |
| `dnsq`, `dnsqr`, `dnsip*`, `dnsname`, `dnsmx`, `dnstxt` | queries and diagnostics |
| `dnsfilter`, `dnstrace`, `random-ip` | stream lookup, delegation tracing, testing |
| `rgbdns-log-report` | daily per-zone query and distinct-client aggregation |
| `*-conf` | service-directory generation |
| `setuidgid`, `multilog`, `tai64n`, `tai64nlocal` | process and logging support |

This composition makes privilege and failure boundaries visible. A compiler
can run with write access to data while the server runs read-only. A recursive
cache can be restarted without touching authority. Diagnostic clients reuse
the packet and client libraries rather than embedding daemon behavior.

```
