---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Operating an authoritative service

## Build, stage, verify, replace

A safe publication cycle separates source editing from serving:

```sh
cd /etc/rgbdns
tinydns-data
tinydns-get example.com A www.example.com
```

In production, compile in a staging directory, run representative exact,
wildcard, delegation, negative, IPv4, IPv6, and large-response queries, then
atomically replace `data.cdb`. Retain the previous known-good database for
rollback. Query the bound service over both UDP and TCP after deployment.

For an ANAME zone, test the two address families and the unaffected apex
record types separately:

```sh
dig @192.0.2.53 example.com A +norecurse
dig @192.0.2.53 example.com AAAA +norecurse
dig @192.0.2.53 example.com SOA +norecurse
dig @192.0.2.53 example.com MX +norecurse
```

The A and AAAA answers should have the apex as their owner and should not
contain a CNAME. The SOA and MX answers should come entirely from zone data.
Repeat the address queries after the target changes and after its TTL expires;
this verifies refresh behavior rather than only the initial lookup. Also test
the chosen recursive endpoint independently, because an authoritative ANAME
lookup cannot succeed when its upstream resolver is unavailable.

Do not expose the recursive service to arbitrary networks by accident. The
default `ALLOW_NETS` is loopback only because an open resolver can be abused
for amplification and can consume local capacity. Likewise, expand AXFR
allowlists only for intended secondaries.

## Case study: cron.sh primary and BuddyNS secondaries

Consider a Debian EC2 instance behind Elastic IP `52.10.53.234`. It is the
editable primary for `cron.sh`, published as `a.ns.cron.sh`, while BuddyNS
copies and serves the zone as a secondary. This arrangement illustrates the
whole operational chain: package installation, zone authority, glue,
single-address AXFR, delegation, supervision, and updates.

At the network boundary, allow public UDP and TCP port 53. UDP carries most
queries; TCP is required both for ordinary retry behavior and for AXFR. Do not
restrict all TCP DNS to the secondary provider. The server distinguishes AXFR
questions on the shared stream and applies a narrow source allow-list only to
those transfers. On EC2 the guest usually sees a private address rather than
the Elastic IP, so listening on `0.0.0.0:53` lets AWS translate traffic for the
public address.

Build the native package on a Debian or Ubuntu machine of the same
architecture, copy it to the server, and install it:

```sh
sudo apt update
sudo apt install -y build-essential cargo debhelper rustc git
git clone https://github.com/querygraph/rgbdns.git
cd rgbdns
packaging/build-deb.sh
scp ../rgbdns_0.1.1_amd64.deb admin@52.10.53.234:/tmp/
ssh admin@52.10.53.234
sudo apt install -y /tmp/rgbdns_0.1.1_amd64.deb
```

Installation creates the non-login `rgbdns` account, the protected
configuration and state directories, and the hardened systemd units. It does
not start a nameserver. That separation prevents package installation from
publishing placeholder data.

The primary source includes the SOA, the in-bailiwick primary and its glue, and
the account-assigned BuddyNS names. In schematic form:

```text
Zcron.sh:a.ns.cron.sh:hostmaster.cron.sh:2026072601:16384:2048:1048576:2560:3600
&cron.sh:52.10.53.234:a.ns.cron.sh:3600
&cron.sh::<BuddyNS name 1>:3600
&cron.sh::<BuddyNS name 2>:3600
&cron.sh::<BuddyNS name 3>:3600
+a.ns.cron.sh:52.10.53.234:3600
```

Replace the placeholders with the names shown in BuddyBoard. The complete,
directly compilable source is in the Debian deployment guide. Store it as
`/root/cron.sh.data`, add the application records, and increment the SOA serial
on every publication. The empty address fields on the BuddyNS NS lines are
intentional: glue for those names belongs to BuddyNS, not `cron.sh`.

BuddyNS publishes the addresses from which its cluster initiates transfers.
Its current documentation says every published source must be allowed. For an
IPv4-only primary, express the published IPv4 addresses as exact `/32`
networks:

```sh
BUDDYNS_AXFR_V4='108.61.224.67/32,116.203.6.3/32'
BUDDYNS_AXFR_V4="$BUDDYNS_AXFR_V4,107.191.99.111/32"
BUDDYNS_AXFR_V4="$BUDDYNS_AXFR_V4,193.109.120.66/32"
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

## Observe the right signals

Useful signals include:

- query and error rate by transport;
- truncated UDP responses and TCP retries;
- SERVFAIL, REFUSED, NXDOMAIN, and validation-failure rates;
- resolver cache capacity and latency percentiles;
- process restarts and file-descriptor use;
- root-hint and trust-anchor freshness;
- ANAME refresh latency, upstream failures, cache misses, and synthesized TTLs;
- time synchronization;
- CDB build identity and deployment time.

High NXDOMAIN volume is not automatically an incident; browsers, typo traffic,
and discovery protocols generate it. A change from baseline paired with
latency or SERVFAIL is more meaningful.

TAI64N log labels make events stable for storage. Convert them for human
display at the edge:

```sh
tail -f main/current | tai64nlocal
```

## Generated Code Fragment Index

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-e5fff4b8cb2b", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-e5fff4b8cb2b: const HEADER_LEN", "sourcePath": "src/cdb.rs", "startLine": 9, "endLine": 9}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4907bb687e44", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-4907bb687e44: const MAX_DATABASE_SIZE", "sourcePath": "src/cdb.rs", "startLine": 10, "endLine": 11}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f318af8bdeaa", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-f318af8bdeaa: fn compile", "sourcePath": "src/cdb.rs", "startLine": 12, "endLine": 61}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f67eebb3c015", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-f67eebb3c015: fn load", "sourcePath": "src/cdb.rs", "startLine": 62, "endLine": 101}
```

```rgbdns-fragment
{"id": "rgbdns-frag-916ec1cbc28e", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-916ec1cbc28e: fn read_entries", "sourcePath": "src/cdb.rs", "startLine": 102, "endLine": 156}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f9047bc1a1a2", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-f9047bc1a1a2: fn decode_record", "sourcePath": "src/cdb.rs", "startLine": 157, "endLine": 210}
```

```rgbdns-fragment
{"id": "rgbdns-frag-bf06479bb119", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-bf06479bb119: fn encode_rdata", "sourcePath": "src/cdb.rs", "startLine": 211, "endLine": 269}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f2d13363a376", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-f2d13363a376: fn decode_name", "sourcePath": "src/cdb.rs", "startLine": 270, "endLine": 296}
```

```rgbdns-fragment
{"id": "rgbdns-frag-ca380a5004ce", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-ca380a5004ce: fn le_u32", "sourcePath": "src/cdb.rs", "startLine": 297, "endLine": 301}
```

```rgbdns-fragment
{"id": "rgbdns-frag-9cc58af3bb02", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-9cc58af3bb02: mod tests", "sourcePath": "src/cdb.rs", "startLine": 302, "endLine": 307}
```

```rgbdns-fragment
{"id": "rgbdns-frag-d600fc524f2b", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-d600fc524f2b: fn exact_cdb_roundtrip_preserves_lookup_semantics", "sourcePath": "src/cdb.rs", "startLine": 308, "endLine": 364}
```

```rgbdns-fragment
{"id": "rgbdns-frag-428b1ce3e4be", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-428b1ce3e4be: fn rejects_truncated_database", "sourcePath": "src/cdb.rs", "startLine": 365, "endLine": 371}
```
