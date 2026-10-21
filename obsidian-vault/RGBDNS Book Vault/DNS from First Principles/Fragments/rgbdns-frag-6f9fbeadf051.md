---
type: "code-fragment"
fragment_id: "rgbdns-frag-6f9fbeadf051"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "fieldnotes.es application zone"
kind: "heading"
start_line: 1510
end_line: 1590
---

# fieldnotes.es application zone

- Fragment ID: `rgbdns-frag-6f9fbeadf051`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1510-1590
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-6f9fbeadf051", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 1510, "endLine": 1590}
```

## Excerpt

<span id="rgbdns-frag-6f9fbeadf051" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6f9fbeadf051: heading fieldnotes.es application zone

````markdown
# fieldnotes.es application zone
Zfieldnotes.es:a.ns.cron.sh:hostmaster.cron.sh:2026073001:16384:2048:1048576:2560:3600
&fieldnotes.es::a.ns.cron.sh:3600
&fieldnotes.es::b.ns.cron.sh:3600
&fieldnotes.es::uz5x6wcwzfbjs8fkmkuchydn9339lf7xbxdmnp038cmyjlgg9sprr2.free.ns.buddyns.com:3600
&fieldnotes.es::uz5dkwpjfvfwb9rh1qj93mtup0gw65s6j7vqqumch0r9gzlu8qxx39.free.ns.buddyns.com:3600
&fieldnotes.es::uz56xw8h7fw656bpfv84pctjbl9rbzbqrw4rpzdhtvzyltpjdmx0zq.free.ns.buddyns.com:3600
```

Increment the affected SOA serial whenever the source changes. Obtain the
current BuddyNS transfer-source CIDRs from BuddyNS and store them in the login
account's protected `buddyns-axfr.env`; provider networks are operational
input, not constants to copy forever from a book.

From the normal `bitnami` shell on the primary, configure the role and its
watched `rgbdns.data` path:

```sh
. "$HOME/buddyns-axfr.env"
PRIMARY_AXFR_NETS="172.31.0.125/32,$BUDDYNS_AXFR_V4"

sudo rgbdns-setup primary \
  --data "$HOME/rgbdns.data" \
  --data-drop "$HOME/rgbdns.data" \
  --data-drop-owner "$(id -un)" \
  --listen-ip 0.0.0.0 --port 53 \
  --allow-nets "$PRIMARY_AXFR_NETS" \
  --query-log 1
```

The secondary transfers both zones because it is advertised for both:

```text
cron.sh
fieldnotes.es
```

From the normal openSUSE login shell:

```sh
. "$HOME/buddyns-axfr.env"

sudo rgbdns-setup secondary \
  --zones "cron.sh fieldnotes.es" \
  --primary 172.31.60.189 \
  --zones-drop "$HOME/rgbdns.zones" \
  --zones-drop-owner "$(id -un)" \
  --listen-ip 0.0.0.0 --port 53 \
  --allow-nets "$BUDDYNS_AXFR_V4" \
  --query-log 1
```

Use the private primary address for VPC AXFR while retaining the public
addresses in delegation. Later, publish complete primary data and the desired
secondary list through temporary names and atomic renames:

```sh
scp rgbdns.data bitnami@52.10.53.234:rgbdns.data.new
ssh bitnami@52.10.53.234 'mv rgbdns.data.new rgbdns.data'

scp rgbdns.zones "$SUSE_USER"@52.38.177.160:rgbdns.zones.new
ssh "$SUSE_USER"@52.38.177.160 'mv rgbdns.zones.new rgbdns.zones'
```

The primary importer compiles a private staged copy and leaves the live CDB
unchanged on failure. The secondary importer validates, normalizes, and
atomically installs the list before starting AXFR. Inspect both boundaries:

```sh
sudo journalctl -u rgbdns-data-import.service \
  -u rgbdns-zones-import.service \
  -u rgbdns-secondary-sync.service -n 100 --no-pager
```

The standalone
[`RGBDNS_SETUP.md`](https://github.com/querygraph/rgbdns/blob/master/docs/RGBDNS_SETUP.md)
walkthrough contains package download and installation, AWS rules, BuddyNS,
delegation, upgrades, complete verification, and troubleshooting commands for
this exact deployment.

For a deployment-level ANAME transfer proof, add an undelegated reserved test
````
