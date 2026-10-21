---
type: "code-fragment"
fragment_id: "rgbdns-frag-09c36a3ee069"
source_path: "docs/DEBIAN.md"
code_note: "DNS from First Principles/Code/docs/DEBIAN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "reconnect after boot"
kind: "heading"
start_line: 537
end_line: 547
---

# reconnect after boot

- Fragment ID: `rgbdns-frag-09c36a3ee069`
- Source file: [[DNS from First Principles/Code/docs/DEBIAN.md.source|docs/DEBIAN.md]]
- Lines: 537-547
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-09c36a3ee069", "codeNote": "DNS from First Principles/Code/docs/DEBIAN.md.source", "sourcePath": "docs/DEBIAN.md", "startLine": 537, "endLine": 547}
```

## Excerpt

<span id="rgbdns-frag-09c36a3ee069" class="rgbdns-fragment-target"></span>
### rgbdns-frag-09c36a3ee069: heading reconnect after boot

````markdown
# reconnect after boot
systemctl is-active rgbdns-tinydns
dig @127.0.0.1 cron.sh SOA +norecurse
```

Monitor at least service state, UDP and TCP queries, SOA serial agreement,
BuddyNS transfer status, disk space, and upcoming package/security updates.
Keep `/root/cron.sh.data` and the exact BuddyNS source list in configuration
management or encrypted backup. A DNS secondary improves serving availability;
it is not a backup of the editable primary source.

````
