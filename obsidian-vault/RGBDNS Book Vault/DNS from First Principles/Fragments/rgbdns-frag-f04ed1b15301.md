---
type: "code-fragment"
fragment_id: "rgbdns-frag-f04ed1b15301"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Full deployment walkthrough three authoritative topologies"
kind: "heading"
start_line: 1428
end_line: 1501
---

# Full deployment walkthrough three authoritative topologies

- Fragment ID: `rgbdns-frag-f04ed1b15301`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1428-1501
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-f04ed1b15301", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 1428, "endLine": 1501}
```

## Excerpt

<span id="rgbdns-frag-f04ed1b15301" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f04ed1b15301: heading Full deployment walkthrough three authoritative topologies

````markdown
## Full deployment walkthrough: three authoritative topologies

This walkthrough installs one editable rgbdns primary and, when selected, one
rgbdns secondary and/or BuddyNS. It deliberately shares one preparation,
publication, and verification path among three useful topologies:

| Topology | Published authorities | AXFR readers of `a` | AXFR readers of `b` |
|---|---|---|---|
| `a + BuddyNS` | `a` and the assigned BuddyNS names | BuddyNS | not applicable |
| `a + b + BuddyNS` | `a`, `b`, and BuddyNS | `b` and BuddyNS | BuddyNS, if configured as an alternate master |
| `a + b` | `a` and `b` | `b` | none |

The common path is intentionally longer than any topology-specific branch.
Choose the topology once, construct the corresponding NS and AXFR lists, then
reuse the same installation and verification commands.

### Names, addresses, and the security boundary

The examples use one service zone with in-bailiwick nameservers:

```text
ZONES="example.net example.org"
PRIMARY_NS=a.ns.example.net
SECONDARY_NS=b.ns.example.net
PRIMARY_PUBLIC_IP=192.0.2.53
SECONDARY_PUBLIC_IP=198.51.100.53
PRIMARY_PRIVATE_IP=10.0.1.10
SECONDARY_PRIVATE_IP=10.0.2.10
```

Replace every documentation address and name. On AWS, bind each daemon to
`0.0.0.0:53`; the guest normally sees its private interface while the Internet
gateway maps its Elastic IP. Use private addresses for AXFR between instances
in the same VPC. Give both instances stable public addresses before publishing
delegation.

Permit public UDP 53 and public TCP 53 in the cloud security group and host
firewall. Ordinary DNS needs both transports, so do not limit all TCP 53 to
secondaries. rgbdns applies `ALLOW_NETS` only to AXFR questions. Separately
allow TCP 53 from the secondary's private address or, preferably on AWS, its
security group.

The examples place `a.ns.example.net` and `b.ns.example.net` inside the served
zone, so the parent needs glue for both. Some deployments use names from a
separate infrastructure zone. For example, `fieldnotes.es` can use
`a.ns.cron.sh` and `b.ns.cron.sh`. In that case:

- put empty address fields on the `fieldnotes.es` NS lines;
- publish the `a.ns.cron.sh` and `b.ns.cron.sh` A records in the `cron.sh`
  zone, not in `fieldnotes.es`;
- create glue at the parent of `cron.sh` when those names are in-bailiwick
  nameservers for `cron.sh`; and
- one packaged secondary instance can synchronize many zones from the same
  primary endpoint.

Consequently, advertise `b` only for zones included in `/var/lib/rgbdns/tinydns/zones`. Add
both `fieldnotes.es` and `cron.sh` when `b` should be authoritative for both;
otherwise retain other working authorities for the omitted infrastructure
zone.

### Worked deployment: `fieldnotes.es` on `a.ns.cron.sh` and `b.ns.cron.sh`

The concrete deployment uses:

| Role | Name | Public address | VPC address |
|---|---|---:|---:|
| Debian primary | `a.ns.cron.sh` | `52.10.53.234` | `172.31.60.189` |
| openSUSE secondary | `b.ns.cron.sh` | `52.38.177.160` | `172.31.0.125` |

The primary serves two zones from one source. `cron.sh` supplies the
nameserver addresses, while `fieldnotes.es` delegates to those names plus the
three assigned BuddyNS authorities:

```text
````
