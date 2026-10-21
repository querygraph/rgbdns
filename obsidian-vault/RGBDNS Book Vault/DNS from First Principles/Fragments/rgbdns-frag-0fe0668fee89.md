---
type: "code-fragment"
fragment_id: "rgbdns-frag-0fe0668fee89"
source_path: "docs/DEBIAN.md"
code_note: "DNS from First Principles/Code/docs/DEBIAN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Upgrades, removal, and troubleshooting"
kind: "heading"
start_line: 678
end_line: 711
---

# Upgrades, removal, and troubleshooting

- Fragment ID: `rgbdns-frag-0fe0668fee89`
- Source file: [[DNS from First Principles/Code/docs/DEBIAN.md.source|docs/DEBIAN.md]]
- Lines: 678-711
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-0fe0668fee89", "codeNote": "DNS from First Principles/Code/docs/DEBIAN.md.source", "sourcePath": "docs/DEBIAN.md", "startLine": 678, "endLine": 711}
```

## Excerpt

<span id="rgbdns-frag-0fe0668fee89" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0fe0668fee89: heading Upgrades, removal, and troubleshooting

````markdown
## Upgrades, removal, and troubleshooting

Package upgrades preserve `/etc/rgbdns/tinydns.env` as a conffile. Beginning
with 0.3.3, an upgrade detects an existing configured role, restarts
`rgbdns-tinydns`, and restores that role's path and timer units. A fresh
unconfigured installation still starts nothing until `rgbdns-setup` records a
role. Managed zone data and optional role files live outside the package
payload. Removing the package stops its units but preserves configuration and
zone state; purge or remove those files explicitly only after taking a backup.

After upgrading a configured host, verify rather than manually reconstructing
unit state:

```sh
systemctl is-active rgbdns-tinydns.service
systemctl status rgbdns-data.path rgbdns-zones.path \
  rgbdns-secondary-sync.timer --no-pager
sudo ss -lntup '( sport = :53 )'
```

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
````
