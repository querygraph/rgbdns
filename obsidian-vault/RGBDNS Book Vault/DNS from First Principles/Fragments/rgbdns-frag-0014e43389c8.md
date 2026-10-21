---
type: "code-fragment"
fragment_id: "rgbdns-frag-0014e43389c8"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "7. Maintain the BuddyNS AXFR source list"
kind: "heading"
start_line: 237
end_line: 265
---

# 7. Maintain the BuddyNS AXFR source list

- Fragment ID: `rgbdns-frag-0014e43389c8`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 237-265
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-0014e43389c8", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 237, "endLine": 265}
```

## Excerpt

<span id="rgbdns-frag-0014e43389c8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0014e43389c8: heading 7. Maintain the BuddyNS AXFR source list

````markdown
## 7. Maintain the BuddyNS AXFR source list

On the workstation, create `buddyns-axfr.env` from BuddyNS's current published
transfer sources:

```sh
printf '%s\n' \
  "BUDDYNS_AXFR_V4='CURRENT_BUDDYNS_CIDRS'" \
  > buddyns-axfr.env
chmod 0600 buddyns-axfr.env
```

It must be one shell assignment whose value is a comma-separated CIDR list,
for example:

```sh
BUDDYNS_AXFR_V4='203.0.113.10/32,203.0.113.11/32'
```

Copy it to both login accounts:

```sh
scp buddyns-axfr.env bitnami@52.10.53.234:
scp buddyns-axfr.env "$SUSE_USER"@52.38.177.160:
```

Treat this as provider-maintained security policy. Refresh it whenever
BuddyNS changes its transfer network.

````
