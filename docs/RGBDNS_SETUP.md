# End-to-end rgbdns setup: `fieldnotes.es`

This walkthrough deploys rgbdns 0.3.1 on two AWS EC2 instances and retains
BuddyNS as an additional secondary network. It covers installation, initial
role configuration, authoritative data, AXFR, atomic primary and secondary
updates, verification, upgrades, and recovery.

The target topology is:

| Role | DNS name | Public address | VPC address | Distribution |
|---|---|---:|---:|---|
| Primary | `a.ns.cron.sh` | `52.10.53.234` | `172.31.60.189` | Debian |
| Secondary | `b.ns.cron.sh` | `52.38.177.160` | `172.31.0.125` | openSUSE |

The example also uses these BuddyNS names:

```text
uz5x6wcwzfbjs8fkmkuchydn9339lf7xbxdmnp038cmyjlgg9sprr2.free.ns.buddyns.com
uz5dkwpjfvfwb9rh1qj93mtup0gw65s6j7vqqumch0r9gzlu8qxx39.free.ns.buddyns.com
uz56xw8h7fw656bpfv84pctjbl9rbzbqrw4rpzdhtvzyltpjdmx0zq.free.ns.buddyns.com
```

Use the names currently assigned in BuddyBoard if they differ. Likewise,
obtain the current BuddyNS AXFR source-address list from BuddyNS rather than
treating an old list as permanent.

## 1. Understand the two zones

`fieldnotes.es` delegates authority to names beneath `cron.sh`:

```text
a.ns.cron.sh
b.ns.cron.sh
```

Consequently, the primary must serve:

- `cron.sh`, which contains the address records for `a.ns.cron.sh` and
  `b.ns.cron.sh`; and
- `fieldnotes.es`, the application zone.

This walkthrough makes `a`, `b`, and BuddyNS authoritative for both zones. If
`b` is not published as an authority for `cron.sh`, it may transfer only
`fieldnotes.es`; keep `cron.sh` out of its list in that case.

## 2. Prepare AWS networking and delegation

Give both instances stable public addresses. Within the VPC, use private
addresses for AXFR from `b` to `a`.

Allow these inbound flows:

- public UDP port 53 to both nameservers;
- public TCP port 53 to both nameservers;
- SSH only from administrative sources; and
- VPC TCP port 53 from `172.31.0.125` to the primary.

Ordinary DNS requires both UDP and TCP. The AWS security group therefore
allows public TCP 53; rgbdns separately applies `ALLOW_NETS` only to AXFR.

At the `.sh` registrar, create child-host/glue records:

```text
a.ns.cron.sh  A  52.10.53.234
b.ns.cron.sh  A  52.38.177.160
```

Delegate `fieldnotes.es` at its registrar to:

```text
a.ns.cron.sh
b.ns.cron.sh
uz5x6wcwzfbjs8fkmkuchydn9339lf7xbxdmnp038cmyjlgg9sprr2.free.ns.buddyns.com
uz5dkwpjfvfwb9rh1qj93mtup0gw65s6j7vqqumch0r9gzlu8qxx39.free.ns.buddyns.com
uz56xw8h7fw656bpfv84pctjbl9rbzbqrw4rpzdhtvzyltpjdmx0zq.free.ns.buddyns.com
```

Do not change delegation until both rgbdns servers and BuddyNS return the same
SOA serial.

## 3. Download the current packages

Run these commands on a workstation with `gh` authenticated to GitHub. They
select the newest successful `master` run through the API, without depending
on a `gh run list --status` option.

Download the Debian artifact:

```sh
DEB_RUN=$(
  gh api --method GET \
    repos/querygraph/rgbdns/actions/workflows/build-deb.yml/runs \
    -f branch=master -f status=success -f per_page=1 \
    --jq '.workflow_runs[0].id'
)

mkdir -p "$HOME/rgbdns-deb"
gh run download "$DEB_RUN" \
  -R querygraph/rgbdns \
  -n rgbdns-debian-amd64 \
  -D "$HOME/rgbdns-deb"
```

Download the openSUSE artifact:

```sh
RPM_RUN=$(
  gh api --method GET \
    repos/querygraph/rgbdns/actions/workflows/build-rpm.yml/runs \
    -f branch=master -f status=success -f per_page=1 \
    --jq '.workflow_runs[0].id'
)

mkdir -p "$HOME/rgbdns-rpm"
gh run download "$RPM_RUN" \
  -R querygraph/rgbdns \
  -n rgbdns-opensuse-leap16-x86_64 \
  -D "$HOME/rgbdns-rpm"
```

The 0.3.1 artifacts are:

```text
rgbdns_0.3.1_amd64.deb
RPMS/x86_64/rgbdns-0.3.1-1.x86_64.rpm
SRPMS/rgbdns-0.3.1-1.src.rpm
```

## 4. Install the Debian package on the primary

Copy the package:

```sh
scp "$HOME/rgbdns-deb/rgbdns_0.3.1_amd64.deb" \
  bitnami@52.10.53.234:/tmp/
```

On the primary:

```sh
sudo apt install /tmp/rgbdns_0.3.1_amd64.deb
sudo systemctl daemon-reload
dpkg-query -W -f='${Package} ${Version}\n' rgbdns
getent passwd rgbdns
```

During an upgrade, retain the existing `/etc/rgbdns/tinydns.env` when `dpkg`
asks about the locally modified conffile. It contains the deployment-specific
AXFR allow-list. A missing `QUERY_LOG` assignment still means request logging
is enabled, although this walkthrough writes it explicitly.

## 5. Install the RPM on the secondary

Set the actual openSUSE login name:

```sh
SUSE_USER=ec2-user
```

Copy the package:

```sh
scp "$HOME/rgbdns-rpm/RPMS/x86_64/rgbdns-0.3.1-1.x86_64.rpm" \
  "$SUSE_USER"@52.38.177.160:/tmp/
```

On the secondary:

```sh
sudo zypper --non-interactive --no-gpg-checks install \
  /tmp/rgbdns-0.3.1-1.x86_64.rpm
sudo systemctl daemon-reload
rpm -q rgbdns
sudo rpm -V rgbdns
getent passwd rgbdns
```

An upgrade from the earlier `ZONES=` layout migrates the list once to
`/etc/rgbdns/zones`. New setup writes the canonical file directly.

## 6. Maintain the BuddyNS AXFR source list

On the workstation, create `buddyns-axfr.env` from BuddyNS's current published
transfer sources:

```sh
printf '%s\n' \
  "BUDDYNS_AXFR_V4='CURRENT_BUDDYNS_CIDRS'" \
  > buddyns-axfr.env
chmod 0600 buddyns-axfr.env
```

It must be one shell assignment whose value is a comma-separated CIDR list,
for example:

```sh
BUDDYNS_AXFR_V4='203.0.113.10/32,203.0.113.11/32'
```

Copy it to both login accounts:

```sh
scp buddyns-axfr.env bitnami@52.10.53.234:
scp buddyns-axfr.env "$SUSE_USER"@52.38.177.160:
```

Treat this as provider-maintained security policy. Refresh it whenever
BuddyNS changes its transfer network.

## 7. Create the complete primary data

On the workstation, create `rgbdns.data`. Increment each SOA serial for every
published change.

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

# Add fieldnotes.es application records below this line.
# +fieldnotes.es:192.0.2.80:300
# +www.fieldnotes.es:192.0.2.80:300
```

The address-bearing `cron.sh` NS lines publish the nameserver address records.
The `fieldnotes.es` NS lines intentionally have empty address fields because
their names belong to the separate `cron.sh` zone.

Copy the initial source to the primary login account:

```sh
scp rgbdns.data bitnami@52.10.53.234:rgbdns.data
```

## 8. Configure the primary role once

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

## 9. Create and configure the secondary zone list

On the workstation, create `rgbdns.zones` with one zone per line:

```text
cron.sh
fieldnotes.es
```

Copy the initial list:

```sh
scp rgbdns.zones "$SUSE_USER"@52.38.177.160:rgbdns.zones
```

On the secondary, from its normal login shell:

```sh
. "$HOME/buddyns-axfr.env"

sudo rgbdns-setup secondary \
  --zones "cron.sh fieldnotes.es" \
  --primary 172.31.60.189 \
  --zones-drop "$HOME/rgbdns.zones" \
  --zones-drop-owner "$(id -un)" \
  --listen-ip 0.0.0.0 \
  --port 53 \
  --allow-nets "$BUDDYNS_AXFR_V4" \
  --query-log 1
```

The private primary address is intentional. AWS-to-AWS AXFR stays inside the
VPC; public DNS still advertises `52.10.53.234` and `52.38.177.160`.

Inspect the result:

```sh
sudo cat /etc/rgbdns/secondary.env
sudo cat /etc/rgbdns/zones
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

`inactive/dead` is correct for a completed `Type=oneshot` service.

## 10. Use ANAME only between upgraded rgbdns peers

rgbdns 0.3.1 preserves private ANAME directives when both AXFR peers run
rgbdns. For an apex hosted by Ghost, the primary source can contain:

```text
Znew-domain.example:a.ns.cron.sh:hostmaster.cron.sh:2026073001:16384:2048:1048576:2560:3600
&new-domain.example::a.ns.cron.sh:3600
&new-domain.example::b.ns.cron.sh:3600
Anew-domain.example:publication.ghost.io:300
```

`axfr-get` negotiates the extension automatically. The primary sends the
private target and TTL only after that capability request; the secondary
validates it, restores the ANAME directive, and performs its own address
flattening. Verify both address families:

```sh
for server in 52.10.53.234 52.38.177.160; do
  dig @"$server" new-domain.example A +norecurse
  dig @"$server" new-domain.example AAAA +norecurse
done
```

Standard AXFR clients receive no private ANAME metadata and cannot reproduce
this behavior. Therefore delegate an ANAME-backed domain only to upgraded
rgbdns authorities such as `a` and `b`. Do not add that domain to BuddyNS
unless it uses standard A/AAAA records instead.

## 11. Configure BuddyNS

In BuddyBoard:

1. Add `cron.sh` and `fieldnotes.es`.
2. Set the public primary/master to `52.10.53.234`, port 53.
3. Run the BuddyNS transfer test for both zones.
4. Require the transferred serials to match the primary.
5. Optionally configure `52.38.177.160` as an alternate master only after
   AXFR from `b` is verified.

The application allow-list permits BuddyNS to AXFR from either rgbdns server.
Do not use the VPC address for BuddyNS; it is outside the VPC.

## 12. Publish later primary changes

Edit the complete `rgbdns.data` on the workstation and increment the affected
SOA serial. Upload through a temporary name, then atomically rename it:

```sh
scp rgbdns.data bitnami@52.10.53.234:rgbdns.data.new &&
ssh bitnami@52.10.53.234 'mv rgbdns.data.new rgbdns.data'
```

The path watcher:

1. verifies that the final path is a regular file, not a symlink;
2. verifies ownership;
3. copies it into a private staging directory;
4. compiles it as the `rgbdns` service user;
5. replaces live `data` and `data.cdb` only after successful compilation; and
6. restarts tinydns.

Inspect the publication:

```sh
ssh bitnami@52.10.53.234 \
  'sudo journalctl -u rgbdns-data-import.service \
     -u rgbdns-tinydns.service -n 100 --no-pager'
```

A malformed or partially uploaded file does not replace the active database.
The `.new` plus rename sequence prevents the watcher from observing an upload
while `scp` is still writing it.

## 13. Publish later secondary-list changes

Edit `rgbdns.zones`, one zone per line. Blank lines and leading `#` comment
lines are accepted. Publish atomically:

```sh
scp rgbdns.zones "$SUSE_USER"@52.38.177.160:rgbdns.zones.new &&
ssh "$SUSE_USER"@52.38.177.160 \
  'mv rgbdns.zones.new rgbdns.zones'
```

The importer rejects symlinks, unexpected ownership, malformed names, and an
empty list. It lowercases names, removes duplicates, atomically installs
`/etc/rgbdns/zones`, and starts synchronization.

Inspect:

```sh
ssh "$SUSE_USER"@52.38.177.160 \
  'sudo journalctl -u rgbdns-zones-import.service \
     -u rgbdns-secondary-sync.service -n 100 --no-pager'
```

Before adding a zone, confirm the primary already serves and transfers it:

```sh
dig @172.31.60.189 new-zone.example SOA +norecurse
dig @172.31.60.189 new-zone.example AXFR
```

Run those commands from the secondary or another authorized VPC source.

## 14. Verify the public deployment

Compare all rgbdns authorities:

```sh
for server in 52.10.53.234 52.38.177.160; do
  for zone in cron.sh fieldnotes.es; do
    dig @"$server" "$zone" SOA +norecurse
    dig @"$server" "$zone" NS +norecurse
    dig +tcp @"$server" "$zone" SOA +norecurse
  done
done
```

Check each BuddyNS authority:

```sh
for ns in \
  uz5x6wcwzfbjs8fkmkuchydn9339lf7xbxdmnp038cmyjlgg9sprr2.free.ns.buddyns.com \
  uz5dkwpjfvfwb9rh1qj93mtup0gw65s6j7vqqumch0r9gzlu8qxx39.free.ns.buddyns.com \
  uz56xw8h7fw656bpfv84pctjbl9rbzbqrw4rpzdhtvzyltpjdmx0zq.free.ns.buddyns.com
do
  dig @"$ns" fieldnotes.es SOA +norecurse
done
```

Check delegation and glue:

```sh
dig fieldnotes.es NS +trace
dig cron.sh NS +trace
dig a.ns.cron.sh A
dig b.ns.cron.sh A
```

Require:

- `status: NOERROR`;
- the `aa` flag on direct authoritative queries;
- identical current SOA serials;
- matching NS RRsets; and
- UDP and TCP agreement.

## 15. Observe request and transfer logs

Request logging is enabled by default. Follow primary activity:

```sh
sudo journalctl -fu rgbdns-tinydns.service \
  -u rgbdns-data-import.service
```

Follow secondary activity:

```sh
sudo journalctl -fu rgbdns-tinydns.service \
  -u rgbdns-zones-import.service \
  -u rgbdns-secondary-sync.service
```

Disable per-request logging only when explicitly intended:

```ini
QUERY_LOG=0
```

Restart tinydns after manually changing that environment file.

## 16. Install, upgrade, or change roles

The package and `rgbdns-setup` have separate responsibilities:

- package installation places binaries and units but does not guess a role;
- `rgbdns-setup primary` records `/etc/rgbdns/data-drop.env`;
- `rgbdns-setup secondary` records `/etc/rgbdns/secondary.env` and the
  secondary zone files; and
- package upgrades from 0.3.1 onward detect that recorded role and restore
  only its picker and timer units.

### Fresh installation as a primary

Install the package, retain any intentionally preseeded `tinydns.env`, and run
the primary setup once:

On Debian:

```sh
sudo apt install ./rgbdns_VERSION_amd64.deb
sudo rgbdns-setup primary \
  --data "$HOME/rgbdns.data" \
  --data-drop "$HOME/rgbdns.data" \
  --data-drop-owner "$(id -un)" \
  --listen-ip 0.0.0.0 \
  --port 53 \
  --allow-nets "$PRIMARY_AXFR_NETS" \
  --query-log 1
```

Verify the role:

```sh
test -f /etc/rgbdns/data-drop.env
systemctl is-enabled rgbdns-data.path
systemctl is-active rgbdns-data.path
systemctl is-active rgbdns-tinydns
```

The two active units are `rgbdns-data.path` and `rgbdns-tinydns.service`.
Secondary synchronization units remain disabled.

### Fresh installation as a secondary

Install the RPM or Debian package, place the initial one-zone-per-line drop
file, and run secondary setup once:

On openSUSE:

```sh
sudo zypper --non-interactive --no-gpg-checks install \
  ./rgbdns-VERSION-1.x86_64.rpm
sudo rgbdns-setup secondary \
  --zones "cron.sh fieldnotes.es" \
  --primary 172.31.60.189 \
  --zones-drop "$HOME/rgbdns.zones" \
  --zones-drop-owner "$(id -un)" \
  --listen-ip 0.0.0.0 \
  --port 53 \
  --allow-nets "$SECONDARY_AXFR_NETS" \
  --query-log 1
```

Every initial zone must transfer before setup activates the combined
secondary database. Verify:

```sh
sudo systemctl is-active rgbdns-tinydns
sudo systemctl is-enabled rgbdns-zones.path
sudo systemctl is-active rgbdns-zones.path
sudo systemctl is-enabled rgbdns-secondary-sync.timer
sudo systemctl is-active rgbdns-secondary-sync.timer
sudo systemctl show rgbdns-secondary-sync.service \
  -p Result -p ExecMainStatus
```

The primary data picker remains disabled.

### Upgrade an already configured host

Install the newer package normally. Keep the existing Debian conffile or RPM
`.rpmnew` behavior so deployment-specific `ALLOW_NETS` survives.

```sh
# Debian
sudo apt install ./rgbdns_NEW_VERSION_amd64.deb

# openSUSE
sudo zypper --non-interactive --no-gpg-checks install \
  ./rgbdns-NEW_VERSION-1.x86_64.rpm
```

On upgrades from 0.3.1 onward, the maintainer script reloads systemd and
restores role automation according to the existing configuration:

| Existing configuration | Restored units |
|---|---|
| `/etc/rgbdns/data-drop.env` | `rgbdns-data.path` |
| `/etc/rgbdns/secondary.env` | `rgbdns-secondary-sync.timer` |
| secondary plus `/etc/rgbdns/zones-drop.env` | timer and `rgbdns-zones.path` |
| no recorded role | none |

Restart authority to run the upgraded binary, then verify the role:

```sh
sudo systemctl restart rgbdns-tinydns
sudo systemctl is-active rgbdns-tinydns
sudo systemctl status rgbdns-data.path rgbdns-zones.path \
  rgbdns-secondary-sync.timer --no-pager
```

An enabled unit that is `inactive` is not picking up changes. The path unit
for the configured role must be `active (waiting)`, and a configured
secondary's timer must be `active (waiting)`.

When upgrading from a release older than 0.3.1, activate the recorded role
once because the older package did not restore it:

```sh
# Primary only
sudo systemctl enable --now rgbdns-data.path

# Secondary only
sudo systemctl enable --now rgbdns-secondary-sync.timer rgbdns-zones.path
```

### Repurpose a primary as a secondary

Prepare a valid `rgbdns.zones` first and ensure every listed zone is
transferable from the new primary. Then run the secondary setup command:

```sh
sudo rgbdns-setup secondary \
  --zones "cron.sh fieldnotes.es" \
  --primary NEW_PRIMARY_ADDRESS \
  --zones-drop "$HOME/rgbdns.zones" \
  --zones-drop-owner "$(id -un)" \
  --listen-ip 0.0.0.0 \
  --port 53 \
  --allow-nets "$SECONDARY_AXFR_NETS" \
  --query-log 1
```

The command removes the primary drop configuration, disables
`rgbdns-data.path`, performs the initial AXFR synchronization, and enables the
secondary timer and zone-list picker. Verify that no primary picker remains:

```sh
test ! -e /etc/rgbdns/data-drop.env
! systemctl is-enabled --quiet rgbdns-data.path
systemctl is-active rgbdns-zones.path
systemctl is-active rgbdns-secondary-sync.timer
sudo cat /etc/rgbdns/secondary.env
sudo cat /etc/rgbdns/zones
```

Do not change registrar delegation until direct queries to the repurposed host
return the expected authoritative serials.

### Repurpose a secondary as a primary

Prepare and validate the complete authoritative `rgbdns.data`, including a
new SOA serial for every zone, then run:

```sh
sudo rgbdns-setup primary \
  --data "$HOME/rgbdns.data" \
  --data-drop "$HOME/rgbdns.data" \
  --data-drop-owner "$(id -un)" \
  --listen-ip 0.0.0.0 \
  --port 53 \
  --allow-nets "$PRIMARY_AXFR_NETS" \
  --query-log 1
```

The command compiles the primary database before activation, removes
secondary configuration, disables the secondary picker and timer, and enables
the primary data picker. Verify:

```sh
test ! -e /etc/rgbdns/secondary.env
test ! -e /etc/rgbdns/zones
! systemctl is-enabled --quiet rgbdns-secondary-sync.timer
! systemctl is-enabled --quiet rgbdns-zones.path
systemctl is-active rgbdns-data.path
systemctl is-active rgbdns-tinydns
```

For either conversion, `rgbdns-setup` is the supported transition mechanism.
Do not manually leave both role configurations or both picker units enabled.

## 17. Troubleshoot by boundary

### AXFR ends with `end of file`

First test ordinary DNS over TCP:

```sh
dig @172.31.60.189 fieldnotes.es SOA +tcp +norecurse
```

Then test AXFR from the authorized secondary:

```sh
dig @172.31.60.189 fieldnotes.es AXFR
```

If private AXFR works but AXFR through `52.10.53.234` does not, keep
`PRIMARY=172.31.60.189`; the private path is the intended AWS-to-AWS route.
BuddyNS continues to use the public primary address.

Check the primary's effective allow-list:

```sh
sudo grep '^ALLOW_NETS=' /etc/rgbdns/tinydns.env
sudo journalctl -u rgbdns-tinydns.service -n 100 --no-pager
```

### A one-shot is `inactive (dead)`

For `rgbdns-secondary-sync.service`, this is success when:

```text
Result=success
ExecMainStatus=0
ActiveState=inactive
SubState=dead
```

### A path upload did nothing

Check the configured path and owner:

```sh
sudo cat /etc/rgbdns/data-drop.env 2>/dev/null || true
sudo cat /etc/rgbdns/zones-drop.env 2>/dev/null || true
systemctl status rgbdns-data.path rgbdns-zones.path --no-pager
```

Ensure the final file is owned by the configured login user and publish with
the `.new` plus rename sequence.

### The unit changed on disk

After a package upgrade:

```sh
sudo systemctl daemon-reload
```

This reloads systemd unit definitions; it does not alter rgbdns configuration
or zone data.

### Confirm listening sockets

```sh
sudo ss -lntup '( sport = :53 )'
```

The authoritative process should own both UDP and TCP port 53. AXFR is handled
through the integrated TCP listener; do not start a second `axfrdns` service
on the same address and port.
