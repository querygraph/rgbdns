---
type: "code-fragment"
fragment_id: "rgbdns-frag-ba22dbc9d1cd"
source_path: "docs/DEBIAN.md"
code_note: "DNS from First Principles/Code/docs/DEBIAN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "etc rgbdns query-report.env"
kind: "heading"
start_line: 415
end_line: 495
---

# etc rgbdns query-report.env

- Fragment ID: `rgbdns-frag-ba22dbc9d1cd`
- Source file: [[DNS from First Principles/Code/docs/DEBIAN.md.source|docs/DEBIAN.md]]
- Lines: 415-495
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-ba22dbc9d1cd", "codeNote": "DNS from First Principles/Code/docs/DEBIAN.md.source", "sourcePath": "docs/DEBIAN.md", "startLine": 415, "endLine": 495}
```

## Excerpt

<span id="rgbdns-frag-ba22dbc9d1cd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ba22dbc9d1cd: heading etc rgbdns query-report.env

````markdown
# /etc/rgbdns/query-report.env
REPORT_TO=deliverable@gmail.com
REPORT_FROM=rgbdns@a.ns.cron.sh
REPORT_SENDMAIL=/usr/sbin/sendmail
```

Enable the timer on the primary only, avoiding duplicate reports from the
secondary:

```sh
sudo systemctl daemon-reload
sudo systemctl enable --now rgbdns-query-report.timer
sudo systemctl start rgbdns-query-report.service
sudo systemctl status rgbdns-query-report.service --no-pager --full
sudo journalctl -u rgbdns-query-report.service -n 50 --no-pager
systemctl list-timers rgbdns-query-report.timer
```

The manual start sends the preceding local calendar day's report immediately.
The timer normally runs shortly after 00:15 with a small randomized delay.
Journald retention must cover the previous day, and `QUERY_LOG=1` must remain
enabled. The timer is deliberately not enabled by package installation because
the operator must first configure a recipient and working mail transport.

There is deliberately no `rgbdns-axfrdns.service` in this package. The
standalone `axfrdns` command remains part of the djbdns-compatible tool suite,
but it cannot bind `52.10.53.234:53` beside `tinydns`. For this one-address
deployment, `tinydns` recognizes AXFR on its TCP socket and invokes the same
bounded AXFR implementation after checking `ALLOW_NETS`. Do not launch a
second `axfrdns` process for this example.

### 5. Verify the primary before delegating

From a machine outside EC2:

```sh
dig @52.10.53.234 cron.sh SOA +norecurse
dig @52.10.53.234 cron.sh NS +norecurse
dig @52.10.53.234 a.ns.cron.sh A +norecurse
dig @52.10.53.234 cron.sh SOA +tcp +norecurse
```

All answers should have the `aa` flag, and UDP and TCP SOA answers should
agree. An AXFR attempt from an address absent from `ALLOW_NETS` should receive
no zone:

```sh
dig @52.10.53.234 cron.sh AXFR
```

To run a positive AXFR test from an administrator host, temporarily add only
that host's public `/32` to `BUDDYNS_AXFR_V4`, rerun `rgbdns-setup`, test, and
immediately restore the BuddyNS-only list. The definitive production test is a
successful transfer initiated by BuddyNS.

### 6. Configure BuddyNS and delegation

In BuddyBoard:

1. Add the zone `cron.sh`.
2. Set its primary/master server to `52.10.53.234` on port 53.
3. Use BuddyNS's Target or transfer test and require a successful AXFR.
4. Record the BuddyNS names assigned to the zone.
5. Make the zone's NS records exactly match the primary plus the selected
   BuddyNS names.

At the `.sh` registrar, create or retain the child-host/glue record
`a.ns.cron.sh = 52.10.53.234`, then delegate `cron.sh` to:

```text
a.ns.cron.sh
uz5x6wcwzfbjs8fkmkuchydn9339lf7xbxdmnp038cmyjlgg9sprr2.free.ns.buddyns.com
uz5dkwpjfvfwb9rh1qj93mtup0gw65s6j7vqqumch0r9gzlu8qxx39.free.ns.buddyns.com
uz56xw8h7fw656bpfv84pctjbl9rbzbqrw4rpzdhtvzyltpjdmx0zq.free.ns.buddyns.com
```

Do not change registrar delegation until BuddyNS reports a successful transfer
and each server answers the new SOA. Parent delegation and the NS RRset inside
the zone must agree.

After delegation propagates, query every authority:
````
