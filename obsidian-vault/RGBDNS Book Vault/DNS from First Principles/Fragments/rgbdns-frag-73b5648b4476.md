---
type: "code-fragment"
fragment_id: "rgbdns-frag-73b5648b4476"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "9. Configure the primary role once"
kind: "heading"
start_line: 303
end_line: 353
---

# 9. Configure the primary role once

- Fragment ID: `rgbdns-frag-73b5648b4476`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 303-353
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-73b5648b4476", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 303, "endLine": 353}
```

## Excerpt

<span id="rgbdns-frag-73b5648b4476" class="rgbdns-fragment-target"></span>
### rgbdns-frag-73b5648b4476: heading 9. Configure the primary role once

````markdown
## 9. Configure the primary role once

Run this from the normal `bitnami` login shell, not a root login shell. Explicit
drop options make the watched owner and path unambiguous:

```sh
. "$HOME/buddyns-axfr.env"
PRIMARY_AXFR_NETS="172.31.0.125/32,$BUDDYNS_AXFR_V4"

sudo rgbdns-setup primary \
  --data "$HOME/rgbdns.data" \
  --data-drop "$HOME/rgbdns.data" \
  --data-drop-owner "$(id -un)" \
  --listen-ip 0.0.0.0 \
  --port 53 \
  --allow-nets "$PRIMARY_AXFR_NETS" \
  --query-log 1
```

`rgbdns-setup` rewrites `/etc/rgbdns/tinydns.env`. Whenever it is rerun, pass
the complete allow-list again; omitting it removes AXFR authorization.

Inspect the result:

```sh
sudo cat /etc/rgbdns/tinydns.env
sudo cat /etc/rgbdns/data-drop.env
sudo systemctl cat rgbdns-data.path
sudo systemctl status rgbdns-data.path --no-pager
sudo systemctl status rgbdns-tinydns --no-pager --full
sudo ss -lntup '( sport = :53 )'
```

Expected drop configuration:

```ini
DATA_DROP=/home/bitnami/rgbdns.data
DATA_DROP_OWNER=bitnami
```

Verify both zones locally:

```sh
for zone in cron.sh fieldnotes.es; do
  dig @127.0.0.1 "$zone" SOA +norecurse
  dig @127.0.0.1 "$zone" NS +norecurse
done
dig @127.0.0.1 a.ns.cron.sh A +norecurse
dig @127.0.0.1 b.ns.cron.sh A +norecurse
```

````
