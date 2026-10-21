---
type: "code-fragment"
fragment_id: "rgbdns-frag-3916572cf2c7"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "openSUSE"
kind: "heading"
start_line: 791
end_line: 871
---

# openSUSE

- Fragment ID: `rgbdns-frag-3916572cf2c7`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 791-871
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-3916572cf2c7", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 791, "endLine": 871}
```

## Excerpt

<span id="rgbdns-frag-3916572cf2c7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3916572cf2c7: heading openSUSE

````markdown
# openSUSE
sudo zypper --non-interactive --no-gpg-checks install \
  ./rgbdns-NEW_VERSION-1.x86_64.rpm
```

On upgrades from 0.3.3 onward, the maintainer script reloads systemd and
restores authority and role automation according to the existing
configuration:

| Existing configuration | Restored units |
|---|---|
| `/etc/rgbdns/data-drop.env` | `rgbdns-tinydns.service` and `rgbdns-data.path` |
| `/etc/rgbdns/secondary.env` | `rgbdns-tinydns.service` and `rgbdns-secondary-sync.timer` |
| secondary plus `/etc/rgbdns/zones-drop.env` | authority, timer, and `rgbdns-zones.path` |
| no recorded role | none |

Verify that the upgrade restarted authority and restored the role:

```sh
sudo systemctl is-active rgbdns-tinydns
sudo systemctl status rgbdns-data.path rgbdns-zones.path \
  rgbdns-secondary-sync.timer --no-pager
```

An enabled unit that is `inactive` is not picking up changes. The path unit
for the configured role must be `active (waiting)`, and a configured
secondary's timer must be `active (waiting)`.

When upgrading to a release older than 0.3.3, authority and automation require
manual activation. Upgrade directly to 0.3.3 or newer instead.

### Repurpose a primary as a secondary

Prepare a valid `rgbdns.zones` first and ensure every listed zone is
transferable from the new primary. Then run the secondary setup command:

```sh
sudo rgbdns-setup secondary \
  --zones "cron.sh fieldnotes.es" \
  --primary NEW_PRIMARY_ADDRESS \
  --zones-drop-owner "$(id -un)" \
  --listen-ip 0.0.0.0 \
  --port 53 \
  --allow-nets "$SECONDARY_AXFR_NETS" \
  --query-log 1
```

The command removes the primary drop configuration, disables
`rgbdns-data.path`, performs the initial AXFR synchronization, and enables the
secondary timer and zone-list picker. Verify that no primary picker remains:

```sh
test ! -e /etc/rgbdns/data-drop.env
! systemctl is-enabled --quiet rgbdns-data.path
systemctl is-active rgbdns-zones.path
systemctl is-active rgbdns-secondary-sync.timer
sudo cat /etc/rgbdns/secondary.env
sudo cat /var/lib/rgbdns/tinydns/zones
```

Do not change registrar delegation until direct queries to the repurposed host
return the expected authoritative serials.

### Repurpose a secondary as a primary

Prepare and validate the complete authoritative `rgbdns.data`, including a
new SOA serial for every zone, then run:

```sh
sudo rgbdns-setup primary \
  --data "$HOME/rgbdns.data" \
  --data-drop "$HOME/rgbdns.data" \
  --data-drop-owner "$(id -un)" \
  --listen-ip 0.0.0.0 \
  --port 53 \
  --allow-nets "$PRIMARY_AXFR_NETS" \
  --query-log 1
```

The command compiles the primary database before activation, removes
secondary configuration, disables the secondary picker and timer, and enables
````
