---
type: "code-fragment"
fragment_id: "rgbdns-frag-d39318b29e35"
source_path: "docs/DEBIAN.md"
code_note: "DNS from First Principles/Code/docs/DEBIAN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Configure a secondary nameserver"
kind: "heading"
start_line: 455
end_line: 517
---

# Configure a secondary nameserver

- Fragment ID: `rgbdns-frag-d39318b29e35`
- Source file: [[DNS from First Principles/Code/docs/DEBIAN.md.source|docs/DEBIAN.md]]
- Lines: 455-517
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-d39318b29e35", "codeNote": "DNS from First Principles/Code/docs/DEBIAN.md.source", "heading": "rgbdns-frag-d39318b29e35: heading Configure a secondary nameserver", "sourcePath": "docs/DEBIAN.md", "startLine": 455, "endLine": 517}
```

## Excerpt

<span id="rgbdns-frag-d39318b29e35" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d39318b29e35: heading Configure a secondary nameserver

```markdown
## Configure a secondary nameserver

The packaged secondary workflow manages one complete zone per host instance.
It fetches the primary over DNS TCP, verifies response identity, authority,
question, record bounds, zone confinement, and matching SOA bookends, then
atomically installs the new tinydns source and CDB.

Configure the secondary:

```sh
sudo rgbdns-setup secondary \
  --zone example.net \
  --primary 192.0.2.54 \
  --listen-ip 198.51.100.10
```

If the primary uses a nonstandard transfer port:

```sh
sudo rgbdns-setup secondary \
  --zone example.net \
  --primary 192.0.2.53:5354 \
  --listen-ip 127.0.0.1 --port 5353
```

Setup writes `/etc/rgbdns/secondary.env`, performs the first transfer, starts
the authoritative service only after that transfer succeeds, and enables
`rgbdns-secondary-sync.timer`. The timer refreshes every five minutes with a
small randomized delay. Failed transfers leave the last successfully compiled
zone active. A successful transfer atomically replaces `data`, compiles
`data.cdb`, and restarts tinydns.

Run or inspect synchronization manually:

```sh
sudo systemctl start rgbdns-secondary-sync.service
systemctl list-timers rgbdns-secondary-sync.timer
journalctl -u rgbdns-secondary-sync.service
```

Change the interval with a systemd drop-in:

```sh
sudo systemctl edit rgbdns-secondary-sync.timer
```

```ini
[Timer]
OnUnitActiveSec=
OnUnitActiveSec=15min
RandomizedDelaySec=1min
```

Then run:

```sh
sudo systemctl daemon-reload
sudo systemctl restart rgbdns-secondary-sync.timer
```

This is periodic AXFR, not SOA-driven NOTIFY/IXFR. Choose an interval that fits
the zone's change rate and the primary's transfer budget.

```
