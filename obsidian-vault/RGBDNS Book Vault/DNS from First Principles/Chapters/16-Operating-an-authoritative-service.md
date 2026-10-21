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
# cron.sh infrastructure zone
Zcron.sh:a.ns.cron.sh:hostmaster.cron.sh:2026073001:16384:2048:1048576:2560:3600
&cron.sh:52.10.53.234:a.ns.cron.sh:3600
&cron.sh:52.38.177.160:b.ns.cron.sh:3600
&cron.sh::uz5x6wcwzfbjs8fkmkuchydn9339lf7xbxdmnp038cmyjlgg9sprr2.free.ns.buddyns.com:3600
&cron.sh::uz5dkwpjfvfwb9rh1qj93mtup0gw65s6j7vqqumch0r9gzlu8qxx39.free.ns.buddyns.com:3600
&cron.sh::uz56xw8h7fw656bpfv84pctjbl9rbzbqrw4rpzdhtvzyltpjdmx0zq.free.ns.buddyns.com:3600

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
zone to the primary source:

```text
Zaname-axfr.test:a.ns.cron.sh:hostmaster.cron.sh:2026073101:16384:2048:1048576:2560:3600
&aname-axfr.test::a.ns.cron.sh:3600
&aname-axfr.test::b.ns.cron.sh:3600
Aaname-axfr.test:publication.ghost.io:300
```

Publish the complete primary data atomically, add `aname-axfr.test` to the
complete secondary drop list, and publish that list atomically. On the
secondary, this line proves negotiated metadata survived AXFR:

```sh
sudo grep '^Aaname-axfr\.test' \
  /var/lib/rgbdns/tinydns/secondary-zones/aname-axfr.test.data
```

Normal queries to each public authority must return standard authoritative A
or AAAA answers with no CNAME and a TTL no greater than 300. Run the standard
AXFR privacy check from the secondary or another authorized VPC host:

```sh
dig @172.31.60.189 aname-axfr.test AXFR |
  grep -E 'TYPE65401|RGA1|publication\.ghost\.io'
```

No output is expected. The private `172.31.60.189` address is not reachable
from an ordinary workstation outside AWS, and the public AXFR endpoint should
remain limited to explicitly authorized source addresses.

### Obtain and install the packages

The GitHub Actions workflows publish architecture-specific artifacts. They are
not APT or Zypper repositories. Install GitHub CLI and authenticate on a
machine allowed to retrieve the artifacts:

```sh
gh auth login
```

Select the newest successful Debian build without depending on the
version-specific `--status` option found only in newer GitHub CLI releases:

```sh
DEB_RUN_ID=$(
  gh run list -R querygraph/rgbdns \
    -w build-deb.yml -b master -L 50 \
    --json databaseId,conclusion \
    --jq '.[] | select(.conclusion == "success") | .databaseId' |
  head -n 1
)
mkdir -p "$HOME/rgbdns-deb"
gh run download "$DEB_RUN_ID" \
  -R querygraph/rgbdns \
  -n rgbdns-debian-amd64 \
  -D "$HOME/rgbdns-deb"
```

On the Debian or Ubuntu primary, install the downloaded package:

```sh
sudo apt install "$HOME"/rgbdns-deb/rgbdns_*_amd64.deb
sudo dpkg --audit
dpkg-query -W -f='${Status} ${Version}\n' rgbdns
```

The package intentionally replaces Debian's djbdns and daemontools command
packages because both suites own paths such as `/usr/bin/tinydns-get` and
`/usr/bin/multilog`. Review APT's removal plan before confirming on a host
that already runs those services. Never use `dpkg --force-overwrite`.

For a selected topology containing `b`, download the newest successful
openSUSE RPM artifact on the Leap 16 secondary:

```sh
RPM_RUN_ID=$(
  gh run list -R querygraph/rgbdns \
    -w build-rpm.yml -b master -L 50 \
    --json databaseId,conclusion \
    --jq '.[] | select(.conclusion == "success") | .databaseId' |
  head -n 1
)
mkdir -p "$HOME/rgbdns-rpm"
gh run download "$RPM_RUN_ID" \
  -R querygraph/rgbdns \
  -n rgbdns-opensuse-leap16-x86_64 \
  -D "$HOME/rgbdns-rpm"
RPM=$(find "$HOME/rgbdns-rpm/RPMS/x86_64" \
  -maxdepth 1 -name 'rgbdns-[0-9]*.x86_64.rpm' -print -quit)
rpm -K "$RPM"
sudo zypper --non-interactive --no-gpg-checks install "$RPM"
sudo rpm -V rgbdns
```

The artifact retains its `RPMS/x86_64` and `SRPMS` directories. Install the
binary package under `RPMS/x86_64`; `SRPMS` contains the source RPM. The
development package is payload-verified but not repository-signed, hence the
explicit `--no-gpg-checks`. The RPM obsoletes and conflicts with RPM packages
named `djbdns` and `daemontools`; inspect Zypper's transaction if either is
installed.

Both packages create the non-login `rgbdns` user and group, protected
configuration under `/etc/rgbdns`, state under
`/var/lib/rgbdns/tinydns`, and hardened systemd units. Installation does not
publish placeholder DNS data or enable authority.

Verify the installed account and units:

```sh
getent passwd rgbdns
getent group rgbdns
systemctl list-unit-files 'rgbdns-*'
```

### Build one primary source file

Start with common application records and one SOA serial. A sortable
`YYYYMMDDNN` serial is convenient; increment it before every publication.
Construct the authoritative NS portion from exactly one topology block below.

Common records:

```text
Zexample.net:a.ns.example.net:hostmaster.example.net:2026072901:16384:2048:1048576:2560:3600
+example.net:192.0.2.80:3600
C*.example.net:example.net:3600
```

For `a + BuddyNS`:

```text
&example.net:192.0.2.53:a.ns.example.net:3600
&example.net::<BuddyNS name 1>:3600
&example.net::<BuddyNS name 2>:3600
&example.net::<BuddyNS name 3>:3600
```

For `a + b + BuddyNS`:

```text
&example.net:192.0.2.53:a.ns.example.net:3600
&example.net:198.51.100.53:b.ns.example.net:3600
&example.net::<BuddyNS name 1>:3600
&example.net::<BuddyNS name 2>:3600
&example.net::<BuddyNS name 3>:3600
```

For `a + b`:

```text
&example.net:192.0.2.53:a.ns.example.net:3600
&example.net:198.51.100.53:b.ns.example.net:3600
```

An `&` line with an address creates the NS record and its address/glue. The
empty address fields on BuddyNS lines are intentional: those names belong to
BuddyNS. Replace the account-specific placeholders with the exact names shown
in BuddyBoard.

Store the assembled source as `/root/rgbdns.data` on the primary. Protect and
compile a disposable copy before changing the service:

```sh
sudo install -o root -g root -m 0600 rgbdns.data /root/rgbdns.data
check_dir=$(mktemp -d)
sudo install -o "$(id -u)" -g "$(id -g)" -m 0600 \
  /root/rgbdns.data "$check_dir/data"
(cd "$check_dir" && tinydns-data)
ls -lh "$check_dir/data.cdb"
rm -r "$check_dir"
```

Compilation proves syntax and semantic consistency, not that the chosen
addresses, delegation, mail policy, or application records are correct.
Compare the new source with the old zone before cutover. AXFR from an existing
authority is the best inventory when allowed; otherwise query all known record
types and names from configuration management.

### Construct the AXFR allow-list once

For a topology containing BuddyNS, copy BuddyBoard's current published
transfer-source addresses into a protected, sourceable file. Express
individual IPv4 sources as `/32` networks:

```sh
sudo install -o root -g root -m 0600 \
  buddyns-axfr.env /etc/rgbdns/buddyns-axfr.env
. /etc/rgbdns/buddyns-axfr.env
```

The file has this form:

```sh
BUDDYNS_AXFR_V4='203.0.113.10/32,203.0.113.11/32'
```

Treat the addresses as provider-maintained data, not constants copied forever
from a book. Reconcile the file with BuddyBoard before a deployment and after
provider network changes.

Choose the primary allow-list:

```sh
# a + BuddyNS
PRIMARY_ALLOW_NETS=$BUDDYNS_AXFR_V4

# a + b + BuddyNS
PRIMARY_ALLOW_NETS="$SECONDARY_PRIVATE_IP/32,$BUDDYNS_AXFR_V4"

# a + b
PRIMARY_ALLOW_NETS="$SECONDARY_PRIVATE_IP/32"
```

Run only the assignment for the selected topology. Never allow the entire VPC
when one stable secondary address or security-group path is sufficient.

### Configure and cut over the primary

First stage configuration without claiming port 53:

```sh
sudo rgbdns-setup primary \
  --data /root/rgbdns.data \
  --listen-ip 0.0.0.0 \
  --port 53 \
  --allow-nets "$PRIMARY_ALLOW_NETS" \
  --no-start
sudo -u rgbdns /usr/lib/rgbdns/compile-zone
sudo systemd-analyze verify \
  /lib/systemd/system/rgbdns-tinydns.service 2>/dev/null ||
sudo systemd-analyze verify \
  /usr/lib/systemd/system/rgbdns-tinydns.service
sudo ls -lah /var/lib/rgbdns/tinydns
sudo cat /etc/rgbdns/tinydns.env
```

Setup watches `rgbdns.data` in the invoking sudo user's home directory. After
editing the complete source locally, publish it atomically:

```sh
scp rgbdns.data a.ns.example.net:rgbdns.data.new
ssh a.ns.example.net 'mv rgbdns.data.new rgbdns.data'
```

`rgbdns-data.path` verifies ownership, compiles a private staged copy, and
replaces the live source and CDB only after successful compilation. Invalid,
partial, or symlinked uploads leave the currently served database unchanged.
Use `--data-drop FILE` and `--data-drop-owner USER` when another destination is
required.

The two unit paths cover Debian-family and openSUSE layouts. An unrelated
legacy-unit warning from `systemd-analyze` does not invalidate a successful
rgbdns unit check.

On a migration host, identify the existing owner of port 53:

```sh
sudo ss -lntup '( sport = :53 )'
```

Stop only the old authoritative services. Do not stop an entire `runsvdir`
tree on a host where it also owns unrelated applications. For a classic
djbdns layout:

```sh
sudo sv down /etc/axfrdns /etc/axfrdns/log
sudo sv down /etc/tinydns /etc/tinydns/log
sudo ss -lntup '( sport = :53 )'
```

Then enable rgbdns:

```sh
sudo systemctl enable --now rgbdns-tinydns.service
sudo systemctl status rgbdns-tinydns.service --no-pager --full
sudo ss -lntup '( sport = :53 )'
```

The authoritative daemon serves normal UDP, normal TCP, and allowed AXFR on
the same port. Do not start the separately packaged `axfrdns` compatibility
command on port 53.

Verify the primary locally and publicly before configuring delegation:

```sh
dig @127.0.0.1 example.net SOA +norecurse
dig @127.0.0.1 example.net NS +norecurse
dig @127.0.0.1 example.net A +norecurse
dig +tcp @127.0.0.1 example.net SOA +norecurse

dig @192.0.2.53 example.net SOA +norecurse
dig @192.0.2.53 example.net NS +norecurse
dig +tcp @192.0.2.53 example.net SOA +norecurse
```

Require `status: NOERROR`, the `aa` flag, the intended serial, the complete NS
set, and correct address records.

### Configure `b` when the topology includes it

The primary must allow the secondary's source address, and the network path
must permit TCP 53, before this step. On the openSUSE secondary:

```sh
sudo ss -lntup '( sport = :53 )'
sudo rgbdns-setup secondary \
  --zone example.net \
  --zone example.org \
  --primary 10.0.1.10 \
  --listen-ip 0.0.0.0
```

For `a + b + BuddyNS`, BuddyNS may read from `b` as an alternate master. In
that case load the same current provider list and pass it to the secondary:

```sh
. /etc/rgbdns/buddyns-axfr.env
sudo rgbdns-setup secondary \
  --zones "example.net example.org" \
  --primary 10.0.1.10 \
  --listen-ip 0.0.0.0 \
  --allow-nets "$BUDDYNS_AXFR_V4"
```

Choose one of those two setup commands. `--zone` is repeatable; `--zones`
accepts a whitespace- or comma-separated list. Setup performs a complete AXFR
for each zone, validates every response and its SOA bookends, and requires one
valid snapshot per zone before initially starting authority. On later runs, a
failed refresh retains that zone's last-known-good snapshot while successful
zones advance. The snapshots are compiled together and the combined CDB is
installed atomically. The randomized five-minute timer does not use NOTIFY or
IXFR.

Setup stores the canonical one-zone-per-line list in `/var/lib/rgbdns/tinydns/zones` and
watches `rgbdns.zones` in the invoking sudo user's home. Manage later changes
as a file and publish them with an atomic rename:

```sh
scp rgbdns.zones b.ns.example.net:rgbdns.zones.new
ssh b.ns.example.net 'mv rgbdns.zones.new rgbdns.zones'
```

`rgbdns-zones.path` validates ownership and contents before replacing the
canonical list and starting synchronization. Invalid, empty, symlinked, or
partially uploaded lists leave the active configuration unchanged. Use
`--zones-drop FILE` and `--zones-drop-owner USER` during setup when the
default home-directory destination is not appropriate.

Check the one-shot synchronization result:

```sh
systemctl show rgbdns-secondary-sync.service \
  -p Result -p ExecMainStatus -p ActiveState -p SubState
```

A successful completed run reads:

```text
Result=success
ExecMainStatus=0
ActiveState=inactive
SubState=dead
```

`inactive/dead` is correct for a finished `Type=oneshot` service. The
`/run/rgbdns` runtime directory and its lock exist only while synchronization
runs; systemd removes and recreates them for each invocation.

Verify service, timer, and answers:

```sh
sudo systemctl enable --now rgbdns-tinydns.service
sudo systemctl enable --now rgbdns-secondary-sync.timer
sudo systemctl enable --now rgbdns-zones.path
systemctl list-timers rgbdns-secondary-sync.timer
systemctl status rgbdns-zones.path
sudo ss -lntup '( sport = :53 )'
dig @127.0.0.1 example.net SOA +norecurse
dig @198.51.100.53 example.net SOA +norecurse
dig +tcp @198.51.100.53 example.net SOA +norecurse
```

Every secondary serial must match its primary counterpart. To force a refresh
after a publication:

```sh
sudo systemctl start rgbdns-secondary-sync.service
sudo journalctl -u rgbdns-secondary-sync.service -n 50 --no-pager
```

### Configure BuddyNS when the topology includes it

In BuddyBoard:

1. add the zone;
2. configure `192.0.2.53:53` as a transfer master;
3. for `a + b + BuddyNS`, optionally add `198.51.100.53:53` as another master;
4. require the provider's transfer test to succeed; and
5. record the exact assigned BuddyNS names and transfer-source addresses.

The source zone's BuddyNS NS records, BuddyBoard's assigned names, and the
eventual parent delegation must agree exactly. A provider transfer test should
succeed before any registrar change.

From an allowed transfer source, or with a controlled temporary test address
added to `ALLOW_NETS`, verify:

```sh
dig +tcp AXFR example.net @192.0.2.53
dig +tcp AXFR example.net @198.51.100.53  # when b permits BuddyNS
```

An unlisted client should receive `REFUSED`. Do not broaden the allow-list
merely to make an arbitrary workstation AXFR test succeed.

### Publish glue and delegation last

Create or verify registrar host objects before adding in-bailiwick
nameservers:

```text
a.ns.example.net = 192.0.2.53
b.ns.example.net = 198.51.100.53   # topologies containing b
```

Then publish the parent delegation matching the selected topology:

- `a + BuddyNS`: `a` plus the assigned BuddyNS names;
- `a + b + BuddyNS`: `a`, `b`, plus the assigned BuddyNS names;
- `a + b`: `a` and `b`.

Do not advertise `b` before it answers the current serial publicly. During a
migration, keep old working secondaries in both the child NS RRset and parent
delegation until new authorities pass UDP, TCP, SOA, and negative-answer
tests. Remove old secondaries in a later serial change after parent updates
have propagated.

Trace the parent and query every authority:

```sh
dig +trace +nodnssec example.net NS
dig @192.0.2.53 example.net SOA +norecurse
dig @198.51.100.53 example.net SOA +norecurse
```

Query each BuddyNS hostname as well when selected. Compare serials, NS RRsets,
and authoritative flags. Also test a known name, a nonexistent name, UDP, and
TCP:

```sh
dig @192.0.2.53 www.example.net A +norecurse
dig @192.0.2.53 does-not-exist.example.net A +norecurse
dig +tcp @192.0.2.53 www.example.net A +norecurse
```

### Publish changes, upgrade, and recover

For each zone change:

1. edit the protected canonical source;
2. increment the affected SOA serial;
3. compile a disposable copy;
4. rerun `rgbdns-setup primary` with the complete allow-list;
5. query the primary;
6. force or await secondary refresh; and
7. compare every authority's serial.

Package upgrades preserve configuration and state. Upgrade a downloaded
Debian package with `apt install /path/package.deb` and an RPM with:

```sh
sudo zypper --non-interactive --no-gpg-checks install \
  /path/to/rgbdns-VERSION-RELEASE.x86_64.rpm
sudo rpm -V rgbdns
```

A fresh package installation deliberately starts no role-specific automation.
Run `rgbdns-setup primary` or `rgbdns-setup secondary` once. Beginning with
0.3.3, upgrades inspect the role already recorded under `/etc/rgbdns`:

- a primary with `data-drop.env` has authority and `rgbdns-data.path`
  restored;
- a secondary has authority and `rgbdns-secondary-sync.timer` restored and,
  when `zones-drop.env` exists, `rgbdns-zones.path` restored; and
- an unconfigured installation remains inactive.

Verify after an upgrade that `rgbdns-tinydns` is active and that a primary
picker is `active (waiting)`, or that both the secondary timer and zone picker
are `active (waiting)`.

To repurpose a host, prepare the complete input for the destination role and
run the opposite `rgbdns-setup` command. A primary-to-secondary conversion
requires a valid zone list and successful initial AXFR. A
secondary-to-primary conversion requires a complete, validated data file.
The setup command removes the old role configuration, disables its picker and
timer, and activates only the new role. Do not convert roles by manually
enabling both sets of units.

If secondary setup fails, it deliberately leaves authority stopped until the
first valid transfer completes. Diagnose in this order:

```sh
sudo systemctl status rgbdns-secondary-sync.service --no-pager --full
sudo journalctl -u rgbdns-secondary-sync.service -n 100 --no-pager
dig +tcp @10.0.1.10 example.net SOA +norecurse
sudo cat /etc/rgbdns/secondary.env
sudo cat /var/lib/rgbdns/tinydns/zones
sudo cat /etc/rgbdns/tinydns.env
```

A connection timeout points toward routes, security groups, or firewalls.
`REFUSED` points toward `ALLOW_NETS` or an unexpected NAT source address. A
validation error points toward the transferred zone. A successful one-shot
with a refused local query means `rgbdns-tinydns.service` is not yet active.

After every reboot or upgrade, verify:

```sh
systemctl is-enabled rgbdns-tinydns.service
systemctl is-active rgbdns-tinydns.service
systemctl list-timers rgbdns-secondary-sync.timer
sudo ss -lntup '( sport = :53 )'
```

Monitor public UDP and TCP answers, unit restarts, timer failures, SOA
convergence, transfer failures, and disk space. Keep the editable primary
source in protected configuration management or backup. Secondary DNS
improves serving availability; it is not a backup of the canonical source.

The distribution-specific deployment guides contain additional AWS,
firewall, SELinux, and troubleshooting detail:
[`docs/DEBIAN.md`](https://github.com/querygraph/rgbdns/blob/master/docs/DEBIAN.md)
and
[`docs/OPENSUSE.md`](https://github.com/querygraph/rgbdns/blob/master/docs/OPENSUSE.md).

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

Each card names an rgbdns fragment. In FirstPair Reader, Show code lists its numbered lines and Open file opens the collocated code note with those lines marked.

```rgbdns-fragment
{"id": "rgbdns-frag-e5fff4b8cb2b", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 9, "endLine": 9}
```

```rgbdns-fragment
{"id": "rgbdns-frag-4907bb687e44", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 10, "endLine": 11}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f318af8bdeaa", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 12, "endLine": 61}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f67eebb3c015", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 62, "endLine": 101}
```

```rgbdns-fragment
{"id": "rgbdns-frag-916ec1cbc28e", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 102, "endLine": 156}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f9047bc1a1a2", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 157, "endLine": 210}
```

```rgbdns-fragment
{"id": "rgbdns-frag-bf06479bb119", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 211, "endLine": 269}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f2d13363a376", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 270, "endLine": 296}
```

```rgbdns-fragment
{"id": "rgbdns-frag-ca380a5004ce", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 297, "endLine": 301}
```

```rgbdns-fragment
{"id": "rgbdns-frag-9cc58af3bb02", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 302, "endLine": 307}
```

```rgbdns-fragment
{"id": "rgbdns-frag-d600fc524f2b", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 308, "endLine": 364}
```

```rgbdns-fragment
{"id": "rgbdns-frag-428b1ce3e4be", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "sourcePath": "src/cdb.rs", "startLine": 365, "endLine": 371}
```
