---
type: "code-fragment"
fragment_id: "rgbdns-frag-5b84d150797c"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Build, stage, verify, replace"
kind: "heading"
start_line: 1043
end_line: 1079
---

# Build, stage, verify, replace

- Fragment ID: `rgbdns-frag-5b84d150797c`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1043-1079
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-5b84d150797c", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-5b84d150797c: heading Build, stage, verify, replace", "sourcePath": "docs/book/rgbdns.md", "startLine": 1043, "endLine": 1079}
```

## Excerpt

<span id="rgbdns-frag-5b84d150797c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5b84d150797c: heading Build, stage, verify, replace

```markdown
## Build, stage, verify, replace

A safe publication cycle separates source editing from serving:

```sh
cd /etc/rgbdns
tinydns-data
tinydns-get example.com A www.example.com
```

In production, compile in a staging directory, run representative exact,
wildcard, delegation, negative, IPv4, IPv6, and large-response queries, then
atomically replace `data.cdb`. Retain the previous known-good database for
rollback. Query the bound service over both UDP and TCP after deployment.

For an ANAME zone, test the two address families and the unaffected apex
record types separately:

```sh
dig @192.0.2.53 example.com A +norecurse
dig @192.0.2.53 example.com AAAA +norecurse
dig @192.0.2.53 example.com SOA +norecurse
dig @192.0.2.53 example.com MX +norecurse
```

The A and AAAA answers should have the apex as their owner and should not
contain a CNAME. The SOA and MX answers should come entirely from zone data.
Repeat the address queries after the target changes and after its TTL expires;
this verifies refresh behavior rather than only the initial lookup. Also test
the chosen recursive endpoint independently, because an authoritative ANAME
lookup cannot succeed when its upstream resolver is unavailable.

Do not expose the recursive service to arbitrary networks by accident. The
default `ALLOW_NETS` is loopback only because an open resolver can be abused
for amplification and can consume local capacity. Likewise, expand AXFR
allowlists only for intended secondaries.

```
