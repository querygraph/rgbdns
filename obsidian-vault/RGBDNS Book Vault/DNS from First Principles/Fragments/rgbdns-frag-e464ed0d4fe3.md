---
type: "code-fragment"
fragment_id: "rgbdns-frag-e464ed0d4fe3"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "17. Install, upgrade, or change roles"
kind: "heading"
start_line: 705
end_line: 785
---

# 17. Install, upgrade, or change roles

- Fragment ID: `rgbdns-frag-e464ed0d4fe3`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 705-785
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-e464ed0d4fe3", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 705, "endLine": 785}
```

## Excerpt

<span id="rgbdns-frag-e464ed0d4fe3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e464ed0d4fe3: heading 17. Install, upgrade, or change roles

````markdown
## 17. Install, upgrade, or change roles

The package and `rgbdns-setup` have separate responsibilities:

- package installation places binaries and units but does not guess a role;
- `rgbdns-setup primary` records `/etc/rgbdns/data-drop.env`;
- `rgbdns-setup secondary` records `/etc/rgbdns/secondary.env` and the
  secondary zone files; and
- package upgrades from 0.3.3 onward detect that recorded role and restore
  authority together with its picker and timer units.

### Fresh installation as a primary

Install the package, retain any intentionally preseeded `tinydns.env`, and run
the primary setup once:

On Debian:

```sh
sudo apt install ./rgbdns_VERSION_amd64.deb
sudo rgbdns-setup primary \
  --data "$HOME/rgbdns.data" \
  --data-drop "$HOME/rgbdns.data" \
  --data-drop-owner "$(id -un)" \
  --listen-ip 0.0.0.0 \
  --port 53 \
  --allow-nets "$PRIMARY_AXFR_NETS" \
  --query-log 1
```

Verify the role:

```sh
test -f /etc/rgbdns/data-drop.env
systemctl is-enabled rgbdns-data.path
systemctl is-active rgbdns-data.path
systemctl is-active rgbdns-tinydns
```

The two active units are `rgbdns-data.path` and `rgbdns-tinydns.service`.
Secondary synchronization units remain disabled.

### Fresh installation as a secondary

Install the RPM or Debian package, place the initial one-zone-per-line drop
file, and run secondary setup once:

On openSUSE:

```sh
sudo zypper --non-interactive --no-gpg-checks install \
  ./rgbdns-VERSION-1.x86_64.rpm
sudo rgbdns-setup secondary \
  --zones "cron.sh fieldnotes.es" \
  --primary 172.31.60.189 \
  --zones-drop-owner "$(id -un)" \
  --listen-ip 0.0.0.0 \
  --port 53 \
  --allow-nets "$SECONDARY_AXFR_NETS" \
  --query-log 1
```

Every initial zone must transfer before setup activates the combined
secondary database. Verify:

```sh
sudo systemctl is-active rgbdns-tinydns
sudo systemctl is-enabled rgbdns-zones.path
sudo systemctl is-active rgbdns-zones.path
sudo systemctl is-enabled rgbdns-secondary-sync.timer
sudo systemctl is-active rgbdns-secondary-sync.timer
sudo systemctl show rgbdns-secondary-sync.service \
  -p Result -p ExecMainStatus
```

The primary data picker remains disabled.

### Upgrade an already configured host

Install the newer package normally. Keep the existing Debian conffile or RPM
`.rpmnew` behavior so deployment-specific `ALLOW_NETS` survives.
````
