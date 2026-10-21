---
type: "code-fragment"
fragment_id: "rgbdns-frag-0b0b247a3dae"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "10. Create and configure the secondary zone list"
kind: "heading"
start_line: 354
end_line: 434
---

# 10. Create and configure the secondary zone list

- Fragment ID: `rgbdns-frag-0b0b247a3dae`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 354-434
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-0b0b247a3dae", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 354, "endLine": 434}
```

## Excerpt

<span id="rgbdns-frag-0b0b247a3dae" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0b0b247a3dae: heading 10. Create and configure the secondary zone list

````markdown
## 10. Create and configure the secondary zone list

On the workstation, create `rgbdns.zones` with one zone per line:

```text
cron.sh
fieldnotes.es
```

On the secondary, from its normal login shell:

```sh
. "$HOME/buddyns-axfr.env"

sudo rgbdns-setup secondary \
  --zones "cron.sh fieldnotes.es" \
  --primary 172.31.60.189 \
  --zones-drop-owner "$(id -un)" \
  --listen-ip 0.0.0.0 \
  --port 53 \
  --allow-nets "$BUDDYNS_AXFR_V4" \
  --query-log 1
```

Setup creates the managed upload directory. Copy the initial list to its
SELinux-compatible state path:

```sh
scp rgbdns.zones \
  "$SUSE_USER"@52.38.177.160:/var/lib/rgbdns/incoming/rgbdns.zones
```

The private primary address is intentional. AWS-to-AWS AXFR stays inside the
VPC; public DNS still advertises `52.10.53.234` and `52.38.177.160`.

Inspect the result:

```sh
sudo cat /etc/rgbdns/secondary.env
sudo cat /var/lib/rgbdns/tinydns/zones
sudo cat /etc/rgbdns/zones-drop.env
sudo systemctl status rgbdns-zones.path --no-pager
sudo systemctl status rgbdns-secondary-sync.timer --no-pager
sudo systemctl status rgbdns-tinydns --no-pager --full
```

Expected files:

```ini
PRIMARY=172.31.60.189
```

```text
cron.sh
fieldnotes.es
```

The initial setup requires every listed zone to transfer successfully because
none has a cached last-known-good snapshot. Later, an individual AXFR failure
retains that zone's cached snapshot while successful zones advance.

Verify:

```sh
sudo systemctl show rgbdns-secondary-sync.service \
  -p Result -p ExecMainStatus -p ActiveState -p SubState

for zone in cron.sh fieldnotes.es; do
  dig @127.0.0.1 "$zone" SOA +norecurse
  dig @127.0.0.1 "$zone" NS +norecurse
done
```

A successful completed one-shot reads:

```text
Result=success
ExecMainStatus=0
ActiveState=inactive
SubState=dead
```
````
