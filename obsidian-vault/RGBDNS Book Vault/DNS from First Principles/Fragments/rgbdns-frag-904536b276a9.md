---
type: "code-fragment"
fragment_id: "rgbdns-frag-904536b276a9"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "End-to-end rgbdns setup `fieldnotes.es`"
kind: "heading"
start_line: 1
end_line: 26
---

# End-to-end rgbdns setup `fieldnotes.es`

- Fragment ID: `rgbdns-frag-904536b276a9`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 1-26
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-904536b276a9", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 1, "endLine": 26}
```

## Excerpt

<span id="rgbdns-frag-904536b276a9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-904536b276a9: heading End-to-end rgbdns setup `fieldnotes.es`

````markdown
# End-to-end rgbdns setup: `fieldnotes.es`

This walkthrough deploys rgbdns 0.6.5 on two AWS EC2 instances and retains
BuddyNS as an additional secondary network. It covers installation, initial
role configuration, authoritative data, AXFR, atomic primary and secondary
updates, verification, upgrades, and recovery.

The target topology is:

| Role | DNS name | Public address | VPC address | Distribution |
|---|---|---:|---:|---|
| Primary | `a.ns.cron.sh` | `52.10.53.234` | `172.31.60.189` | Debian |
| Secondary | `b.ns.cron.sh` | `52.38.177.160` | `172.31.0.125` | openSUSE |

The example also uses these BuddyNS names:

```text
uz5x6wcwzfbjs8fkmkuchydn9339lf7xbxdmnp038cmyjlgg9sprr2.free.ns.buddyns.com
uz5dkwpjfvfwb9rh1qj93mtup0gw65s6j7vqqumch0r9gzlu8qxx39.free.ns.buddyns.com
uz56xw8h7fw656bpfv84pctjbl9rbzbqrw4rpzdhtvzyltpjdmx0zq.free.ns.buddyns.com
```

Use the names currently assigned in BuddyBoard if they differ. Likewise,
obtain the current BuddyNS AXFR source-address list from BuddyNS rather than
treating an old list as permanent.

````
