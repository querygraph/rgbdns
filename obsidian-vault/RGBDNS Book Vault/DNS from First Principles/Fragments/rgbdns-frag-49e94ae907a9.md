---
type: "code-fragment"
fragment_id: "rgbdns-frag-49e94ae907a9"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "2. Prepare AWS networking and delegation"
kind: "heading"
start_line: 46
end_line: 80
---

# 2. Prepare AWS networking and delegation

- Fragment ID: `rgbdns-frag-49e94ae907a9`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 46-80
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-49e94ae907a9", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 46, "endLine": 80}
```

## Excerpt

<span id="rgbdns-frag-49e94ae907a9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-49e94ae907a9: heading 2. Prepare AWS networking and delegation

````markdown
## 2. Prepare AWS networking and delegation

Give both instances stable public addresses. Within the VPC, use private
addresses for AXFR from `b` to `a`.

Allow these inbound flows:

- public UDP port 53 to both nameservers;
- public TCP port 53 to both nameservers;
- SSH only from administrative sources; and
- VPC TCP port 53 from `172.31.0.125` to the primary.

Ordinary DNS requires both UDP and TCP. The AWS security group therefore
allows public TCP 53; rgbdns separately applies `ALLOW_NETS` only to AXFR.

At the `.sh` registrar, create child-host/glue records:

```text
a.ns.cron.sh  A  52.10.53.234
b.ns.cron.sh  A  52.38.177.160
```

Delegate `fieldnotes.es` at its registrar to:

```text
a.ns.cron.sh
b.ns.cron.sh
uz5x6wcwzfbjs8fkmkuchydn9339lf7xbxdmnp038cmyjlgg9sprr2.free.ns.buddyns.com
uz5dkwpjfvfwb9rh1qj93mtup0gw65s6j7vqqumch0r9gzlu8qxx39.free.ns.buddyns.com
uz56xw8h7fw656bpfv84pctjbl9rbzbqrw4rpzdhtvzyltpjdmx0zq.free.ns.buddyns.com
```

Do not change delegation until both rgbdns servers and BuddyNS return the same
SOA serial.

````
