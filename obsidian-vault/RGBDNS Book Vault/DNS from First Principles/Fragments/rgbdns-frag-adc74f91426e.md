---
type: "code-fragment"
fragment_id: "rgbdns-frag-adc74f91426e"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "11. Use ANAME only between upgraded rgbdns peers"
kind: "heading"
start_line: 438
end_line: 473
---

# 11. Use ANAME only between upgraded rgbdns peers

- Fragment ID: `rgbdns-frag-adc74f91426e`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 438-473
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-adc74f91426e", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 438, "endLine": 473}
```

## Excerpt

<span id="rgbdns-frag-adc74f91426e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-adc74f91426e: heading 11. Use ANAME only between upgraded rgbdns peers

````markdown
## 11. Use ANAME only between upgraded rgbdns peers

rgbdns 0.3.3 preserves private ANAME directives when both AXFR peers run
rgbdns. For an apex hosted by Ghost, the primary source can contain:

```text
Znew-domain.example:a.ns.cron.sh:hostmaster.cron.sh:2026073001:16384:2048:1048576:2560:3600
&new-domain.example::a.ns.cron.sh:3600
&new-domain.example::b.ns.cron.sh:3600
Anew-domain.example:publication.ghost.io:300
```

`axfr-get` negotiates the extension automatically. The primary sends the
private target and TTL only after that capability request; the secondary
validates it, restores the ANAME directive, and performs its own address
flattening. Verify both address families:

```sh
for server in 52.10.53.234 52.38.177.160; do
  dig @"$server" new-domain.example A +norecurse
  dig @"$server" new-domain.example AAAA +norecurse
done
```

Standard AXFR clients receive no private ANAME metadata and cannot reproduce
this behavior. Therefore delegate an ANAME-backed domain only to upgraded
rgbdns authorities such as `a` and `b`. Do not add that domain to BuddyNS
unless it uses standard A/AAAA records instead.

### Prove ANAME-over-AXFR end to end without public delegation

Use the reserved `.test` namespace so this check cannot affect public DNS.
Append this independent zone to the primary `rgbdns.data` and publish it
through the atomic primary drop flow:

```text
````
