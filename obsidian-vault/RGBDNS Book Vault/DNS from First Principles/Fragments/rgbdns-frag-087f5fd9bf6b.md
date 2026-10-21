---
type: "code-fragment"
fragment_id: "rgbdns-frag-087f5fd9bf6b"
source_path: "README.md"
code_note: "DNS from First Principles/Code/README.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Related private services"
kind: "heading"
start_line: 49
end_line: 111
---

# Related private services

- Fragment ID: `rgbdns-frag-087f5fd9bf6b`
- Source file: [[DNS from First Principles/Code/README.md.source|README.md]]
- Lines: 49-111
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-087f5fd9bf6b", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 49, "endLine": 111}
```

## Excerpt

<span id="rgbdns-frag-087f5fd9bf6b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-087f5fd9bf6b: heading Related private services

````markdown
## Related private services

Production configuration and application code are intentionally maintained
outside this public implementation repository:

- [`alexy/cronsh`](https://github.com/alexy/cronsh) owns the private
  consolidated `rgbdns.data`, `rgbdns.zones`, BuddyNS policy, and deployment
  workflow for `a.ns.cron.sh` and `b.ns.cron.sh`.
- [`querygraph/wishfully`](https://github.com/querygraph/wishfully) owns the
  private Wishfully Vercel control plane and CLI. It submits reviewed DNS-state
  pull requests to `alexy/cronsh`.

No production zone data, deployment credential, or Wishfully application
secret belongs in this repository.

The recursive client commands read `DNSCACHEIP` (a comma-separated list of IP
or `IP:port` endpoints) when set, otherwise they use `/etc/resolv.conf`.

rgbdns supports private ANAME directives for CNAME-like apex hosting without
placing an invalid CNAME on the wire:

```text
.example.com:192.0.2.53:ns1.example.com
Aexample.com:customer.blog-host.example:300
```

The `A` line resolves the target and synthesizes authoritative A and AAAA
answers owned by `example.com`; 300 seconds is the TTL cap. It may coexist
with the apex SOA, NS, MX, and TXT records, but not with A, AAAA, or CNAME at
the same owner. ANAME resolution uses `DNSCACHEIP` or `/etc/resolv.conf`.

ANAME directives survive AXFR between rgbdns peers through an explicitly
negotiated private extension. `axfr-get` requests it automatically; ordinary
AXFR clients receive only standard DNS records and no private metadata.

The standards-track proposal in
[`ietf/draft-khrabrov-dnsop-aname-axfr-00.xml`](ietf/draft-khrabrov-dnsop-aname-axfr-00.xml)
defines a portable ANAME RR, native AXFR/IXFR behavior, capability signaling,
DNSSEC and failure handling, and migration from rgbdns's experimental
encoding. Render it with `make -C ietf`; see [`ietf/README.md`](ietf/README.md)
for review and submission guidance.

The `*-conf` commands generate djbdns-style service directories. They reference
rgbdns's own `setuidgid` and `multilog` binaries by absolute path, so
daemontools is not a runtime dependency. `multilog t ./main` writes TAI64N
timestamps to `main/current`; optional `s<size>` and `n<count>` arguments set
the rotation threshold and retained-file count. Daemons continue to write
diagnostics to stderr, allowing the same binaries to work under daemontools,
systemd, containers, or another supervisor.

`tinydns` also writes one original-compatible request record to stderr by
default. The raw record deliberately has no timestamp, so systemd-journald or
`multilog t` can timestamp and rotate the same stream. Set `QUERY_LOG=0`, or
pass `rgbdns-setup --query-log 0`, only when per-request logging is
intentionally disabled.

`rgbdns-log-report` aggregates that stream by authoritative zone, including
subdomain queries, and reports total queries plus distinct client/resolver IP
addresses in descending-total order. Packages include an opt-in daily systemd
timer that reads the preceding local day from journald and delivers the report
through a sendmail-compatible mail transport. DNS query counts are not HTTP
pageviews or unique people.

````
