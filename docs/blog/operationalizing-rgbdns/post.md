# rgbdns is operational: Debian, openSUSE, and ANAME across AXFR

*July 2026 — rgbdns 0.3.3*

![An openSUSE chameleon launches an RPM package spacecraft toward the Moon while a red Debian spiral forms a galaxy in deep space.](operationalizing-rgbdns-headboard.png)

The first rgbdns release established the larger idea: rebuild the djbdns
program family in memory-safe Rust without losing the small-program design,
the tinydns data format, or the ability to understand the entire authoritative
serving path. The newest release turns that implementation into something an
operator can deploy and keep running.

rgbdns 0.3.3 ships installable Debian and openSUSE packages, systemd services
for primary and secondary authority, atomic file-drop workflows for zone data,
multi-zone AXFR synchronization, and query logging compatible with both the
system journal and `multilog`. It also carries a working implementation of
ANAME address flattening that preserves the ANAME source record between
upgraded rgbdns authorities during AXFR.

That last part is now also a standards proposal. The complete design has been
submitted as the individual Internet-Draft
[Address-specific DNS Aliases (ANAME) and Zone Transfer](https://datatracker.ietf.org/doc/draft-khrabrov-dnsop-aname-axfr/).
It is an active proposal for discussion, not an approved RFC or an IETF
endorsement.

## From a package to an operating nameserver

The Debian package is intended for a Debian or Ubuntu primary. Download the
current `.deb` artifact, then install the local file with an explicit path:

```sh
sudo apt install ./rgbdns_0.3.3_amd64.deb
```

The openSUSE package provides the same binaries, service account, configuration
layout, and role tooling:

```sh
sudo zypper install ./rgbdns-0.3.3-1.x86_64.rpm
```

Both packages deliberately install an inert service. Installation creates the
`rgbdns` account and systemd units, but it does not guess whether the machine
is a primary, a secondary, or neither. The role is selected once with
`rgbdns-setup`.

For a primary at `a.ns.cron.sh`, the editable source can arrive as
`rgbdns.data` in the administrator's home directory:

```sh
sudo rgbdns-setup primary \
  --data-drop /home/bitnami/rgbdns.data \
  --data-drop-owner bitnami \
  --listen-ip 0.0.0.0 \
  --allow-nets 172.31.0.125/32
```

The path unit watches the drop location. A complete file is validated,
compiled into CDB, installed atomically, and activated without exposing a
partially written database. Publishing another zone is consequently a source
control and file-transfer operation, not an interactive edit on the server:

```sh
scp rgbdns.data.new primary.example:rgbdns.data.new
ssh primary.example 'mv rgbdns.data.new rgbdns.data'
```

For the openSUSE secondary at `b.ns.cron.sh`, the corresponding one-time setup
names the primary's private VPC address and a drop location for the desired
zone list:

```sh
sudo rgbdns-setup secondary \
  --zones-drop-owner ec2-user \
  --primary 172.31.60.189 \
  --listen-ip 0.0.0.0
```

The operator publishes one zone name per line:

```text
fieldnotes.es
aname-axfr.test
```

As on the primary, upload to a temporary name and rename only after the copy
finishes:

```sh
scp rgbdns.zones.new \
  secondary.example:/var/lib/rgbdns/incoming/rgbdns.zones.new
ssh secondary.example \
  'mv /var/lib/rgbdns/incoming/rgbdns.zones.new /var/lib/rgbdns/incoming/rgbdns.zones'
```

The secondary imports the list, transfers each zone, validates the result,
rebuilds its combined serving database, and keeps last-known-good data for a
zone whose refresh fails. One broken zone therefore does not make healthy
zones disappear.

Package upgrades preserve operator-owned configuration. Debian uses conffile
semantics; RPM uses `%config(noreplace)`. Since 0.3.3, an upgrade also restores
the configured authority and role-specific path or timer units, so installing
a newer package does not leave a previously running nameserver stopped.

## ANAME is an alias that can live at the apex

A CNAME cannot normally coexist with the SOA, NS, and other records required
at a zone apex. Hosting platforms nevertheless want customers to point an
apex such as `fieldnotes.es` at a managed target whose addresses can change.
Providers solve this today under names such as CNAME flattening, ALIAS, or
ANAME.

rgbdns expresses the source intent directly in tinydns data syntax:

```text
Zfieldnotes.es:a.ns.cron.sh:hostmaster.fieldnotes.es:2026073103:16384:2048:1048576:2560:3600
&fieldnotes.es::a.ns.cron.sh:3600
&fieldnotes.es::b.ns.cron.sh:3600
Afieldnotes.es:publication.ghost.io:300
```

The uppercase `A` is an rgbdns ANAME directive: its owner is
`fieldnotes.es`, its target is `publication.ghost.io`, and 300 seconds is the
TTL ceiling. A normal resolver never needs to understand a private record.
When it asks for `A` or `AAAA`, each authoritative rgbdns server resolves the
target and returns ordinary address records at the ANAME owner. SOA, NS, MX,
TXT, and other apex data continue to coexist normally.

The subtle operational problem is transfer. Copying only today's synthesized
addresses to a secondary loses the target relationship and makes the primary's
temporary cache contents look like zone source. Copying an unknown private
type without negotiation risks collision with another implementation's
private encoding.

rgbdns therefore negotiates its experimental transfer extension explicitly.
An upgraded secondary sends the `RGA1` capability in EDNS option 65001. An
upgraded primary may then carry the ANAME source through private-use
TYPE65401, whose RDATA begins with `RGA1` and whose TTL carries the configured
ceiling. The secondary validates that envelope, reconstructs the uppercase
`A` directive, and resolves the target independently. Ordinary AXFR clients
receive an ordinary transfer without the private metadata.

The deployed values are experimental. The Internet-Draft requests assigned
ANAME and EDNS values as `TBD1` and `TBD2`; it does not pretend that 65401 and
65001 are permanent allocations.

## Verify the system at its boundaries

On either authority, ordinary clients should see an authoritative address
answer, not the transfer representation:

```sh
dig @127.0.0.1 fieldnotes.es A +norecurse
dig @127.0.0.1 fieldnotes.es SOA +norecurse
```

On the secondary, the retained source proves that ANAME intent survived the
transfer:

```sh
sudo grep '^Afieldnotes\.es' \
  /var/lib/rgbdns/tinydns/secondary-zones/fieldnotes.es.data
```

The primary-to-secondary AXFR check belongs on the authorized VPC path, not on
an arbitrary workstation outside AWS:

```sh
dig @172.31.60.189 fieldnotes.es AXFR
```

Do not widen the AXFR allow-list merely to make a public test convenient. The
public checks are ordinary UDP and TCP DNS queries to `a.ns.cron.sh` and
`b.ns.cron.sh`; the transfer check runs from an authorized secondary address.

The result is a deliberately small operating model: one editable data file on
the primary, one declarative zone list on each secondary, atomic activation,
and independent authorities that can preserve and execute the same ANAME
intent.

## Read the implementation and the proposal

- [Live Internet-Draft](https://datatracker.ietf.org/doc/draft-khrabrov-dnsop-aname-axfr/)
- [Complete deployment walkthrough](https://github.com/querygraph/rgbdns/blob/master/docs/RGBDNS_SETUP.md)
- [Debian operations guide](https://github.com/querygraph/rgbdns/blob/master/docs/DEBIAN.md)
- [openSUSE operations guide](https://github.com/querygraph/rgbdns/blob/master/docs/OPENSUSE.md)
- [Draft source and publication artifacts](https://github.com/querygraph/rgbdns/tree/master/ietf)
- [rgbdns source](https://github.com/querygraph/rgbdns)
- [DNS from First Principles](https://firstpair.org/books/rgbdns/)

The Internet-Draft is reproduced in full as Appendix C of the book so that the
protocol argument, wire format, transfer rules, security considerations, and
implementation experience travel with the operational account.
