---
type: "code-fragment"
fragment_id: "rgbdns-frag-4f1724eb1f69"
source_path: "docs/OPENSUSE.md"
code_note: "DNS from First Principles/Code/docs/OPENSUSE.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Reconnect after boot"
kind: "heading"
start_line: 390
end_line: 399
---

# Reconnect after boot

- Fragment ID: `rgbdns-frag-4f1724eb1f69`
- Source file: [[DNS from First Principles/Code/docs/OPENSUSE.md.source|docs/OPENSUSE.md]]
- Lines: 390-399
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-4f1724eb1f69", "codeNote": "DNS from First Principles/Code/docs/OPENSUSE.md.source", "sourcePath": "docs/OPENSUSE.md", "startLine": 390, "endLine": 399}
```

## Excerpt

<span id="rgbdns-frag-4f1724eb1f69" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4f1724eb1f69: heading Reconnect after boot

````markdown
# Reconnect after boot.
systemctl is-active rgbdns-tinydns
dig @127.0.0.1 cron.sh SOA +norecurse
```

Monitor the systemd unit, public UDP and TCP answers, BuddyNS transfer status,
SOA convergence, disk space, and Leap security updates. Back up
`/root/cron.sh.data` and the transfer-source policy. A secondary DNS network
improves availability but is not a backup of the editable source.

````
