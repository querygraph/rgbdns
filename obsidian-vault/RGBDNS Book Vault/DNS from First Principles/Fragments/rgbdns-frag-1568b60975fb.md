---
type: "code-fragment"
fragment_id: "rgbdns-frag-1568b60975fb"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Append every remaining 32 from BuddyNS's current list"
kind: "heading"
start_line: 1142
end_line: 1194
---

# Append every remaining 32 from BuddyNS's current list

- Fragment ID: `rgbdns-frag-1568b60975fb`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1142-1194
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-1568b60975fb", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-1568b60975fb: heading Append every remaining 32 from BuddyNS's current list", "sourcePath": "docs/book/rgbdns.md", "startLine": 1142, "endLine": 1194}
```

## Excerpt

<span id="rgbdns-frag-1568b60975fb" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1568b60975fb: heading Append every remaining 32 from BuddyNS's current list

```markdown
# Append every remaining /32 from BuddyNS's current list.
sudo rgbdns-setup primary \
  --data /root/cron.sh.data \
  --listen-ip 0.0.0.0 --port 53 \
  --allow-nets "$BUDDYNS_AXFR_V4"
```

Recheck BuddyNS's source list before deployment and after provider network
changes. The provider's nameserver names are account-assigned as well; use
BuddyBoard rather than treating the names in this example as global
constants.

`rgbdns-setup` validates and copies the source, compiles `data.cdb`, writes the
service environment, enables the service at boot, and starts or restarts
`rgbdns-tinydns.service`. There is no separate packaged AXFR service in this
topology. Although the `axfrdns` compatibility command remains installed, a
second process cannot share `52.10.53.234:53`; the authoritative process
dispatches allowed AXFR questions to the same bounded AXFR engine.

Before changing delegation, verify ordinary UDP and TCP service:

```sh
dig @52.10.53.234 cron.sh SOA +norecurse
dig @52.10.53.234 cron.sh NS +norecurse
dig @52.10.53.234 a.ns.cron.sh A +norecurse
dig @52.10.53.234 cron.sh SOA +tcp +norecurse
systemctl is-enabled rgbdns-tinydns
systemctl is-active rgbdns-tinydns
```

In BuddyBoard, add `cron.sh`, set `52.10.53.234:53` as its primary, and require
the transfer test to succeed. Configure the primary zone's NS RRset and the
registrar delegation with the same BuddyNS names. Because `a.ns.cron.sh` lies
inside the delegated zone, the `.sh` registrar also needs the child-host glue
`a.ns.cron.sh = 52.10.53.234`. Transfer success should precede delegation;
otherwise the new secondaries may be authoritative but empty or stale.

After propagation, query every delegated authority and compare SOA serials.
For subsequent changes, edit `/root/cron.sh.data`, increment the serial, and
rerun the same `rgbdns-setup primary` command with the complete allow-list.
Compilation precedes restart, and the new process loads one zone snapshot for
both normal answers and transfers.

Systemd keeps the foreground process alive with `Restart=on-failure` and starts
the enabled unit after reboot. Operations should monitor unit state, public UDP
and TCP answers, serial convergence at BuddyNS, transfer failures, and disk
space. The editable source still needs its own protected backup: secondary DNS
is availability infrastructure, not configuration backup.

The full allow-list, command-by-command deployment, AWS rules, BuddyBoard
sequence, delegation checks, and troubleshooting procedure live in the
[`docs/DEBIAN.md` deployment guide](https://github.com/querygraph/rgbdns/blob/master/docs/DEBIAN.md).

```
