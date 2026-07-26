---
type: "code-fragment"
fragment_id: "rgbdns-frag-fcd83bef23f0"
source_path: "docs/DEBIAN.md"
code_note: "DNS from First Principles/Code/docs/DEBIAN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Upgrades, removal, and troubleshooting"
kind: "heading"
start_line: 540
end_line: 561
---

# Upgrades, removal, and troubleshooting

- Fragment ID: `rgbdns-frag-fcd83bef23f0`
- Source file: [[DNS from First Principles/Code/docs/DEBIAN.md.source|docs/DEBIAN.md]]
- Lines: 540-561
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-fcd83bef23f0", "codeNote": "DNS from First Principles/Code/docs/DEBIAN.md.source", "heading": "rgbdns-frag-fcd83bef23f0: heading Upgrades, removal, and troubleshooting", "sourcePath": "docs/DEBIAN.md", "startLine": 540, "endLine": 561}
```

## Excerpt

<span id="rgbdns-frag-fcd83bef23f0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-fcd83bef23f0: heading Upgrades, removal, and troubleshooting

```markdown
## Upgrades, removal, and troubleshooting

Package upgrades preserve `/etc/rgbdns/tinydns.env` as a conffile and do not
automatically enable a service. Managed zone data and optional role files live
outside the package payload. Removing the package stops its units but preserves
configuration and zone state; purge or remove those files explicitly only
after taking a backup.

Common failures:

- `Address already in use`: another resolver or rgbdns unit owns the same
  address and port. Remember that tinydns listens on UDP and TCP.
- `Permission denied` while binding: inspect the unit's capability settings
  and any local systemd override.
- `AXFR server returned an error`: verify the zone name, SOA, primary endpoint,
  source-address allow-list, routing, and TCP firewall.
- `fatal` while compiling: run `tinydns-data` in the state directory and
  correct the reported data-file syntax.
- secondary retains old data: inspect the sync journal. Failed validation never
  replaces the active zone.

Use `systemctl cat UNIT` to include local overrides when diagnosing a host.
```
