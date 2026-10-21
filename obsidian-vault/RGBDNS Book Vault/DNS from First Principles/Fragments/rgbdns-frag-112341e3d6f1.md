---
type: "code-fragment"
fragment_id: "rgbdns-frag-112341e3d6f1"
source_path: "docs/OPENSUSE.md"
code_note: "DNS from First Principles/Code/docs/OPENSUSE.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "etc rgbdns query-report.env"
kind: "heading"
start_line: 299
end_line: 379
---

# etc rgbdns query-report.env

- Fragment ID: `rgbdns-frag-112341e3d6f1`
- Source file: [[DNS from First Principles/Code/docs/OPENSUSE.md.source|docs/OPENSUSE.md]]
- Lines: 299-379
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-112341e3d6f1", "codeNote": "DNS from First Principles/Code/docs/OPENSUSE.md.source", "sourcePath": "docs/OPENSUSE.md", "startLine": 299, "endLine": 379}
```

## Excerpt

<span id="rgbdns-frag-112341e3d6f1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-112341e3d6f1: heading etc rgbdns query-report.env

````markdown
# /etc/rgbdns/query-report.env
REPORT_TO=deliverable@gmail.com
REPORT_FROM=rgbdns@a.ns.cron.sh
REPORT_SENDMAIL=/usr/sbin/sendmail
```

Enable it on the primary only:

```sh
sudo systemctl daemon-reload
sudo systemctl enable --now rgbdns-query-report.timer
sudo systemctl start rgbdns-query-report.service
sudo systemctl status rgbdns-query-report.service --no-pager --full
sudo journalctl -u rgbdns-query-report.service -n 50 --no-pager
systemctl list-timers rgbdns-query-report.timer
```

The service reads the preceding local calendar day from journald. Keep
`QUERY_LOG=1` and sufficient journal retention. Package installation does not
enable the timer before mail delivery is configured.

Do not launch a separate `axfrdns` process. The standalone compatibility
command is installed, but it cannot own the same TCP address and port.
`tinydns` serves ordinary UDP and TCP DNS and dispatches allowed AXFR requests
through the same TCP listener.

### Verify before delegation

From outside the EC2 network:

```sh
dig @52.10.53.234 cron.sh SOA +norecurse
dig @52.10.53.234 cron.sh NS +norecurse
dig @52.10.53.234 a.ns.cron.sh A +norecurse
dig @52.10.53.234 cron.sh SOA +tcp +norecurse
dig @52.10.53.234 cron.sh AXFR
```

The normal answers must carry the authoritative flag and agree over UDP and
TCP. The final command should not expose the zone from an address absent from
`ALLOW_NETS`. Install `bind-utils` with zypper if the test host lacks `dig`.

In BuddyBoard:

1. Add `cron.sh`.
2. Set `52.10.53.234:53` as its primary.
3. Require BuddyNS's transfer test to complete successfully.
4. Record the names assigned to the zone.
5. Keep the zone's NS RRset synchronized with the registrar delegation.

At the `.sh` registrar, configure child-host glue
`a.ns.cron.sh = 52.10.53.234`, then delegate to `a.ns.cron.sh` and the assigned
BuddyNS names. Do not change delegation until BuddyNS has transferred the
current serial.

Verify every authority after propagation:

```sh
dig cron.sh NS +trace
dig @a.ns.cron.sh cron.sh SOA +norecurse
for ns in \
  uz5x6wcwzfbjs8fkmkuchydn9339lf7xbxdmnp038cmyjlgg9sprr2.free.ns.buddyns.com \
  uz5dkwpjfvfwb9rh1qj93mtup0gw65s6j7vqqumch0r9gzlu8qxx39.free.ns.buddyns.com \
  uz56xw8h7fw656bpfv84pctjbl9rbzbqrw4rpzdhtvzyltpjdmx0zq.free.ns.buddyns.com
do
  dig "@$ns" cron.sh SOA +norecurse +short
done
```

All four authorities must converge on the same SOA serial.

### Publish updates and survive reboots

Edit the protected source, increment its serial, and rerun the identical setup
command with the complete current BuddyNS list. Compilation must succeed before
systemd restarts the server:

```sh
sudoedit /root/cron.sh.data
sudo rgbdns-setup primary \
  --data /root/cron.sh.data \
````
