---
type: "code-fragment"
fragment_id: "rgbdns-frag-a2269ad59adf"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "One suite, small purposes"
kind: "heading"
start_line: 584
end_line: 605
---

# One suite, small purposes

- Fragment ID: `rgbdns-frag-a2269ad59adf`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 584-605
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-a2269ad59adf", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-a2269ad59adf: heading One suite, small purposes", "sourcePath": "docs/book/rgbdns.md", "startLine": 584, "endLine": 605}
```

## Excerpt

<span id="rgbdns-frag-a2269ad59adf" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a2269ad59adf: heading One suite, small purposes

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
| `*-conf` | service-directory generation |
| `setuidgid`, `multilog`, `tai64n`, `tai64nlocal` | process and logging support |

This composition makes privilege and failure boundaries visible. A compiler
can run with write access to data while the server runs read-only. A recursive
cache can be restarted without touching authority. Diagnostic clients reuse
the packet and client libraries rather than embedding daemon behavior.

```
