---
type: "code-fragment"
fragment_id: "rgbdns-frag-e3ee440024e4"
source_path: "docs/DEBIAN.md"
code_note: "DNS from First Principles/Code/docs/DEBIAN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Configure a secondary nameserver"
kind: "heading"
start_line: 548
end_line: 628
---

# Configure a secondary nameserver

- Fragment ID: `rgbdns-frag-e3ee440024e4`
- Source file: [[DNS from First Principles/Code/docs/DEBIAN.md.source|docs/DEBIAN.md]]
- Lines: 548-628
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-e3ee440024e4", "codeNote": "DNS from First Principles/Code/docs/DEBIAN.md.source", "sourcePath": "docs/DEBIAN.md", "startLine": 548, "endLine": 628}
```

## Excerpt

<span id="rgbdns-frag-e3ee440024e4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e3ee440024e4: heading Configure a secondary nameserver

````markdown
## Configure a secondary nameserver

The packaged secondary workflow manages a list of zones from one primary
endpoint. It fetches each zone separately over DNS TCP, verifies response
identity, authority, question, record bounds, zone confinement, and matching
SOA bookends, retains the last valid snapshot of any zone whose refresh fails,
then compiles and atomically installs one combined CDB. A newly configured zone
must transfer successfully before the secondary can activate it.

Configure the secondary:

```sh
sudo rgbdns-setup secondary \
  --zone example.net \
  --zone example.org \
  --primary 192.0.2.54 \
  --listen-ip 198.51.100.10
```

To permit another secondary provider to AXFR the validated zone from this
secondary, add its exact transfer-source CIDRs:

```sh
sudo rgbdns-setup secondary \
  --zones "example.net example.org" \
  --primary 192.0.2.54 \
  --listen-ip 198.51.100.10 \
  --allow-nets 203.0.113.10/32,203.0.113.11/32
```

Ordinary UDP and TCP answers remain public; `--allow-nets` restricts only
AXFR. Use the provider's current published transfer-source list.

If the primary uses a nonstandard transfer port:

```sh
sudo rgbdns-setup secondary \
  --zones "example.net,example.org" \
  --primary 192.0.2.53:5354 \
  --listen-ip 127.0.0.1 --port 5353
```

Setup writes the one-zone-per-line canonical list to `/var/lib/rgbdns/tinydns/zones` and
the `PRIMARY=` endpoint to `/etc/rgbdns/secondary.env`. It also watches
`/var/lib/rgbdns/incoming/rgbdns.zones`. Override that location or owner with
`--zones-drop FILE` and `--zones-drop-owner USER`.
Setup performs every initial transfer, starts the authoritative service only
after every zone has a valid snapshot, and enables both the five-minute sync
timer and `rgbdns-zones.path`.

Manage later additions as a file and publish it atomically:

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

The path unit validates ownership and syntax, lowercases names, removes
duplicates, atomically installs `/var/lib/rgbdns/tinydns/zones`, and starts synchronization.
An invalid or partial list cannot replace the canonical list. Upload through a
temporary name as shown so the watcher only observes the final rename.

During refreshes, a failed zone retains its last-known-good snapshot while
successfully transferred zones advance. A newly listed zone must obtain its
first valid snapshot before the combined database can include the new list.
The run compiles the snapshots together, atomically replaces `data.cdb`, and
restarts tinydns.

Run or inspect synchronization manually:

```sh
sudo systemctl start rgbdns-secondary-sync.service
systemctl list-timers rgbdns-secondary-sync.timer
systemctl status rgbdns-zones.path
````
