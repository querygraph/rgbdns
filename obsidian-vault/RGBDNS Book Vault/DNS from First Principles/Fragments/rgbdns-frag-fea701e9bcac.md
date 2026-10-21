---
type: "code-fragment"
fragment_id: "rgbdns-frag-fea701e9bcac"
source_path: "docs/OPENSUSE.md"
code_note: "DNS from First Principles/Code/docs/OPENSUSE.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Upgrade, remove, and troubleshoot"
kind: "heading"
start_line: 466
end_line: 525
---

# Upgrade, remove, and troubleshoot

- Fragment ID: `rgbdns-frag-fea701e9bcac`
- Source file: [[DNS from First Principles/Code/docs/OPENSUSE.md.source|docs/OPENSUSE.md]]
- Lines: 466-525
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-fea701e9bcac", "codeNote": "DNS from First Principles/Code/docs/OPENSUSE.md.source", "sourcePath": "docs/OPENSUSE.md", "startLine": 466, "endLine": 525}
```

## Excerpt

<span id="rgbdns-frag-fea701e9bcac" class="rgbdns-fragment-target"></span>
### rgbdns-frag-fea701e9bcac: heading Upgrade, remove, and troubleshoot

````markdown
## Upgrade, remove, and troubleshoot

Upgrade a locally supplied RPM with:

```sh
sudo zypper --non-interactive --no-gpg-checks install \
  /tmp/rgbdns-NEW_VERSION.x86_64.rpm
rpm -V rgbdns
```

The `%config(noreplace)` environment file is preserved across upgrades. Zone
state and `/root/cron.sh.data` are not package payloads. Beginning with 0.3.3,
an upgrade detects an existing configured role, restarts `rgbdns-tinydns`,
and restores that role's path and timer units. A fresh unconfigured install
remains inactive until `rgbdns-setup` records a role.

Verify the packaged lifecycle after an upgrade:

```sh
systemctl is-active rgbdns-tinydns.service
systemctl status rgbdns-data.path rgbdns-zones.path \
  rgbdns-secondary-sync.timer --no-pager
sudo ss -lntup '( sport = :53 )'
```

Stop and remove the software without deleting configuration or zone state:

```sh
sudo systemctl disable --now \
  rgbdns-secondary-sync.timer rgbdns-tinydns.service
sudo zypper --non-interactive remove rgbdns
```

Back up and explicitly remove `/etc/rgbdns` and `/var/lib/rgbdns` only when
retiring the service.

Common failures:

- `Address already in use`: inspect `ss` and stop or rebind the conflicting
  resolver.
- port 53 permission failure: inspect the unit's capability settings and local
  overrides with `systemctl cat rgbdns-tinydns`.
- AXFR failure: check the zone, current BuddyNS source list, TCP security-group
  and firewalld rules, routing, and `ALLOW_NETS`.
- compilation failure: run `sudo -u rgbdns /usr/lib/rgbdns/compile-zone` and
  correct the reported source line.
- service denial under SELinux: inspect the audit log and label/customize only
  the denied path or operation.
- stale secondary: inspect `journalctl -u rgbdns-secondary-sync.service`;
  rejected transfers never overwrite the last valid data.

Useful diagnostics:

```sh
sudo systemd-analyze verify \
  /usr/lib/systemd/system/rgbdns-tinydns.service
sudo journalctl -u rgbdns-tinydns -b
sudo ss -lntup '( sport = :53 )'
sudo rpm -V rgbdns
```
````
