---
type: "code-fragment"
fragment_id: "rgbdns-frag-853d81ccef41"
source_path: "docs/OPENSUSE.md"
code_note: "DNS from First Principles/Code/docs/OPENSUSE.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Configure an rgbdns secondary"
kind: "heading"
start_line: 400
end_line: 465
---

# Configure an rgbdns secondary

- Fragment ID: `rgbdns-frag-853d81ccef41`
- Source file: [[DNS from First Principles/Code/docs/OPENSUSE.md.source|docs/OPENSUSE.md]]
- Lines: 400-465
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-853d81ccef41", "codeNote": "DNS from First Principles/Code/docs/OPENSUSE.md.source", "sourcePath": "docs/OPENSUSE.md", "startLine": 400, "endLine": 465}
```

## Excerpt

<span id="rgbdns-frag-853d81ccef41" class="rgbdns-fragment-target"></span>
### rgbdns-frag-853d81ccef41: heading Configure an rgbdns secondary

````markdown
## Configure an rgbdns secondary

The package can instead manage a list of zones transferred from one primary:

```sh
sudo rgbdns-setup secondary \
  --zone example.net \
  --zone example.org \
  --primary 192.0.2.54 \
  --listen-ip 198.51.100.10
```

Setup performs every initial AXFR before starting authority, enables
`rgbdns-tinydns.service` and `rgbdns-secondary-sync.timer`, writes the canonical
one-zone-per-line list to `/var/lib/rgbdns/tinydns/zones`, and watches
`/var/lib/rgbdns/incoming/rgbdns.zones`. Use `--zones-drop FILE` and
`--zones-drop-owner USER` to override that destination.

To permit an additional secondary provider to transfer the validated zone
from this secondary, pass its exact AXFR source CIDRs:

```sh
sudo rgbdns-setup secondary \
  --zones "example.net example.org" \
  --primary 192.0.2.54 \
  --listen-ip 198.51.100.10 \
  --allow-nets 203.0.113.10/32,203.0.113.11/32
```

Use the provider's current published transfer-source list. Ordinary UDP and
TCP answers remain public; `--allow-nets` restricts only AXFR.

Publish later zone-list changes atomically:

```text
fieldnotes.es
example.net
example.org
```

```sh
scp rgbdns.zones secondary.example:/var/lib/rgbdns/incoming/rgbdns.zones.new
ssh secondary.example \
  'mv /var/lib/rgbdns/incoming/rgbdns.zones.new /var/lib/rgbdns/incoming/rgbdns.zones'
```

`rgbdns-zones.path` starts a protected importer after the final rename. The
importer rejects symlinks, unexpected ownership, malformed names, and empty
lists; normalizes and deduplicates valid names; atomically replaces
`/var/lib/rgbdns/tinydns/zones`; and starts AXFR synchronization. A failed refresh retains
that zone's last-known-good snapshot while successful zones advance. A newly
listed zone must transfer successfully before the new combined list can be
activated.

Inspect or invoke synchronization:

```sh
sudo systemctl start rgbdns-secondary-sync.service
systemctl list-timers rgbdns-secondary-sync.timer
systemctl status rgbdns-zones.path
journalctl -u rgbdns-zones-import.service \
  -u rgbdns-secondary-sync.service
```

This is periodic AXFR, not NOTIFY/IXFR.

````
