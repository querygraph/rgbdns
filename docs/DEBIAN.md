# Installing rgbdns on Debian with systemd

The Debian package installs the complete rgbdns command suite, three systemd
units, a dedicated unprivileged account, and `rgbdns-setup`. Installation alone
does not publish a DNS service. The administrator must choose a primary or
secondary role and provide the corresponding data.

## Build and install the package

Use a current Debian or Ubuntu build host with Rust, Cargo, debhelper, and the
ordinary C build tools:

```sh
sudo apt update
sudo apt install build-essential cargo debhelper rustc
git clone https://github.com/querygraph/rgbdns.git
cd rgbdns
packaging/build-deb.sh
sudo apt install ../rgbdns_0.1.1_$(dpkg --print-architecture).deb
```

`packaging/build-deb.sh` calls `dpkg-buildpackage --build=binary --no-sign`.
Debian's package builder runs the release build and complete Rust test suite.
The resulting package is architecture-specific because it contains native Rust
binaries. Build it on the same Debian architecture as the destination, or use
a proper Debian cross-build environment.

The install step discovers every binary target through `cargo metadata`; the
Debian rules do not maintain a second, manually synchronized program list.

## Cloud package build and verification

The `Build Debian package` GitHub Actions workflow runs for relevant changes on
pull requests and `master`, and can also be started manually:

```sh
gh workflow run build-deb.yml --ref master
gh run watch
```

The workflow builds the native debhelper package on Ubuntu 24.04 with the
current stable Rust toolchain. It then:

1. inspects the package control metadata and file table with `dpkg-deb`;
2. rejects `lintian` errors;
3. installs the package in a clean Ubuntu 24.04 container;
4. verifies dpkg's installed state, every Cargo binary, service unit, and
   packaged helper; and
5. uploads the `.deb` as the `rgbdns-debian-amd64` workflow artifact.

Download a completed build and inspect it locally:

```sh
gh run download RUN_ID -n rgbdns-debian-amd64 -D dist/cloud-deb
dpkg-deb --info dist/cloud-deb/rgbdns_*_amd64.deb
dpkg-deb --contents dist/cloud-deb/rgbdns_*_amd64.deb
```

The workflow passes `-d` to `dpkg-buildpackage` because Rust comes from the
pinned Actions toolchain instead of Ubuntu's `cargo` and `rustc` packages.
Debhelper and all other packaging tools still come from Ubuntu packages.

The package creates:

- system user and group `rgbdns`;
- configuration directory `/etc/rgbdns`, owned by `root:rgbdns`;
- state directory `/var/lib/rgbdns/tinydns`, owned by `rgbdns:rgbdns`;
- commands in `/usr/bin` and the setup command in `/usr/sbin`;
- systemd units for authoritative DNS with integrated AXFR and secondary refresh.

The account has no login shell. Services bind privileged port 53 with only
`CAP_NET_BIND_SERVICE`; they do not run as root. The units make the rest of the
filesystem read-only, hide home directories and most process information,
remove privilege escalation, lock the process personality, deny writable
executable memory, limit address families, and grant write access only to the
managed zone state.

## Prepare a primary nameserver

rgbdns uses the tinydns text format. Start from the packaged example:

```sh
sudo install -m 0644 \
  /usr/share/doc/rgbdns/examples/data \
  /root/example.net.data
sudo editor /root/example.net.data
```

A minimal zone needs an SOA, authoritative NS records, and address records for
in-zone nameserver names:

```text
Zexample.net:ns1.example.net:hostmaster.example.net:2026072301:16384:2048:1048576:2560:3600
&example.net:192.0.2.53:ns1.example.net:3600
+ns1.example.net:192.0.2.53:3600
+www.example.net:192.0.2.80:300
```

Replace all documentation addresses. The SOA administrator field uses a DNS
name (`hostmaster.example.net` represents `hostmaster@example.net`). Increment
the serial whenever the zone changes.

Configure, compile, enable, and start the primary:

```sh
sudo rgbdns-setup primary \
  --data /root/example.net.data \
  --listen-ip 192.0.2.53
```

The setup command creates the account if necessary, installs the source as
`/var/lib/rgbdns/tinydns/data`, writes `/etc/rgbdns/tinydns.env`, compiles
`data.cdb` as the service user, reloads systemd, and enables
`rgbdns-tinydns.service`. Re-running the command safely replaces the managed
configuration.

To validate before starting anything, add `--no-start`, then inspect:

```sh
sudo -u rgbdns /usr/lib/rgbdns/compile-zone
sudo systemd-analyze verify \
  /lib/systemd/system/rgbdns-tinydns.service
sudo systemctl start rgbdns-tinydns
```

After editing the managed data file directly, compile and restart:

```sh
sudo -u rgbdns /usr/lib/rgbdns/compile-zone
sudo systemctl restart rgbdns-tinydns
```

## Serve AXFR to secondary nameservers

`tinydns` handles ordinary DNS and AXFR through the same TCP listener. AXFR is
disabled unless an explicit client allow-list is configured.

AXFR has no TSIG implementation in this release. Its security boundary is the
explicit source-address CIDR allow-list, so use narrow secondary addresses and
enforce the same policy in the host and perimeter firewalls.

Allow two secondary hosts to transfer from the ordinary DNS endpoint:

```sh
sudo rgbdns-setup primary \
  --data /root/example.net.data \
  --listen-ip 192.0.2.53 \
  --allow-nets 198.51.100.10/32,2001:db8:100::10/128
```

For a local lab:

```sh
sudo rgbdns-setup primary \
  --data /root/example.net.data \
  --listen-ip 127.0.0.1 --port 5353 \
  --allow-nets 127.0.0.1/32
```

The generated `/etc/rgbdns/tinydns.env` includes `ALLOW_NETS`. Protect that
file as configuration even though the allow-list is not a secret.

For nftables, a narrowly scoped IPv4 rule resembles:

```nft
ip saddr { 198.51.100.10, 198.51.100.11 } \
  ip daddr 192.0.2.53 tcp dport 53 accept
```

Permit UDP and TCP 53 to the public authoritative endpoint. The application
allow-list restricts AXFR; adjust the example to the host's existing table and
chain rather than pasting it blindly.

Test a transfer from an allowed secondary:

```sh
dig +tcp AXFR example.net @192.0.2.53
# or:
axfr-get example.net 192.0.2.53 /tmp/example.data /tmp/example.data.tmp
```

An unlisted source is disconnected without receiving zone contents.

## Complete deployment: cron.sh primary with BuddyNS secondaries

This example makes the EC2 host behind Elastic IP `52.10.53.234` the primary
for `cron.sh` and uses BuddyNS as the public secondary network. The values below
match the `cron.sh` delegation observed on 2026-07-26:

- primary: `a.ns.cron.sh` at `52.10.53.234`;
- BuddyNS secondaries:
  `uz5x6wcwzfbjs8fkmkuchydn9339lf7xbxdmnp038cmyjlgg9sprr2.free.ns.buddyns.com`,
  `uz5dkwpjfvfwb9rh1qj93mtup0gw65s6j7vqqumch0r9gzlu8qxx39.free.ns.buddyns.com`,
  and
  `uz56xw8h7fw656bpfv84pctjbl9rbzbqrw4rpzdhtvzyltpjdmx0zq.free.ns.buddyns.com`.

BuddyNS names are account-assigned. Use the names shown in BuddyBoard if they
differ; do not copy these names to another account or zone. BuddyNS also
changes its transfer-source inventory over time. Recheck its
[zone-transfer instructions](https://www.buddyns.com/support/setup/zone-transfer/pro/)
before deploying or whenever BuddyNS announces a network change.

### 1. Prepare the host and network

Attach Elastic IP `52.10.53.234` to the EC2 instance. The guest normally sees
its private address rather than the Elastic IP, so this example listens on
`0.0.0.0`; AWS performs the public-address translation.

Before starting rgbdns, find and stop any service already owning DNS ports:

```sh
sudo ss -lntup '( sport = :53 )'
systemctl status systemd-resolved named unbound dnsmasq 2>/dev/null || true
```

Do not disable the host's resolver merely because `systemd-resolved` exists.
It normally binds a loopback address and can coexist with a service bound to a
specific private address, but it conflicts with `0.0.0.0:53`. Resolve any
collision deliberately and confirm `/etc/resolv.conf` still provides working
recursive DNS for package installation and ANAME lookups.

The EC2 security group must allow:

- UDP 53 from `0.0.0.0/0` for ordinary authoritative DNS;
- TCP 53 from `0.0.0.0/0` for DNS-over-TCP and AXFR on the same listener;
- SSH only from the administrator's trusted addresses.

If IPv6 is configured and delegated, add equivalent `::/0` DNS rules. A host
firewall must allow the same traffic. Do not restrict all TCP 53 traffic to
BuddyNS: ordinary DNS clients must be able to retry over TCP. rgbdns applies
the BuddyNS source allow-list only when the TCP question is AXFR.

### 2. Build and install the Debian package

On an amd64 Debian or Ubuntu build machine:

```sh
sudo apt update
sudo apt install -y build-essential cargo debhelper rustc git
git clone https://github.com/querygraph/rgbdns.git
cd rgbdns
packaging/build-deb.sh
dpkg-deb --info ../rgbdns_0.1.1_amd64.deb
```

Copy the package to the EC2 host, then install it there:

```sh
scp ../rgbdns_0.1.1_amd64.deb admin@52.10.53.234:/tmp/
ssh admin@52.10.53.234
sudo apt update
sudo apt install -y /tmp/rgbdns_0.1.1_amd64.deb
dpkg-query -W rgbdns
```

For an arm64 instance, build an arm64 package in an arm64 Debian environment
and substitute `_arm64.deb`. Do not copy a Termux/Android Cargo binary into a
Debian package.

Package installation creates the `rgbdns` account and directories but neither
enables nor starts DNS. That prevents an empty example zone from becoming
public accidentally.

### 3. Create the cron.sh primary zone

Create `/root/cron.sh.data` as the canonical editable source:

```sh
sudo install -m 0600 /dev/null /root/cron.sh.data
sudo editor /root/cron.sh.data
```

Use this starting zone, replacing or adding application records as required:

```text
Zcron.sh:a.ns.cron.sh:hostmaster.cron.sh:2026072601:16384:2048:1048576:2560:3600
&cron.sh:52.10.53.234:a.ns.cron.sh:3600
&cron.sh::uz5x6wcwzfbjs8fkmkuchydn9339lf7xbxdmnp038cmyjlgg9sprr2.free.ns.buddyns.com:3600
&cron.sh::uz5dkwpjfvfwb9rh1qj93mtup0gw65s6j7vqqumch0r9gzlu8qxx39.free.ns.buddyns.com:3600
&cron.sh::uz56xw8h7fw656bpfv84pctjbl9rbzbqrw4rpzdhtvzyltpjdmx0zq.free.ns.buddyns.com:3600
+a.ns.cron.sh:52.10.53.234:3600
```

The `&` line for `a.ns.cron.sh` publishes both its NS record and IPv4 glue.
The BuddyNS `&` lines have an empty address field because their address records
belong to BuddyNS zones. Add web, mail, TXT, CAA, or other records below these
authority records. Increment the SOA serial for every published change; the
date-plus-counter form above permits 99 revisions on 2026-07-26.

Compile a disposable copy before changing the live service:

```sh
work=$(mktemp -d)
sudo cp /root/cron.sh.data "$work/data"
sudo chown "$(id -u):$(id -g)" "$work/data"
(cd "$work" && tinydns-data && tinydns-get soa cron.sh)
rm -rf "$work"
```

### 4. Allow every BuddyNS IPv4 transfer source

As of 2026-07-26, BuddyNS publishes these IPv4 transfer sources:

```text
108.61.224.67
116.203.6.3
107.191.99.111
193.109.120.66
5.223.55.119
192.184.93.99
103.25.56.55
216.73.156.203
37.143.61.179
195.20.17.193
45.77.29.133
116.203.0.64
167.88.161.228
199.195.249.208
104.244.78.122
```

BuddyNS explicitly requires allowing all of its transfer sources. Convert each
address to an exact `/32`, configure the primary, and start it:

```sh
BUDDYNS_AXFR_V4='108.61.224.67/32,116.203.6.3/32,107.191.99.111/32,193.109.120.66/32,5.223.55.119/32,192.184.93.99/32,103.25.56.55/32,216.73.156.203/32,37.143.61.179/32,195.20.17.193/32,45.77.29.133/32,116.203.0.64/32,167.88.161.228/32,199.195.249.208/32,104.244.78.122/32'
sudo rgbdns-setup primary \
  --data /root/cron.sh.data \
  --listen-ip 0.0.0.0 \
  --port 53 \
  --allow-nets "$BUDDYNS_AXFR_V4"
```

This copies the source to `/var/lib/rgbdns/tinydns/data`, compiles
`data.cdb`, writes `/etc/rgbdns/tinydns.env`, enables
`rgbdns-tinydns.service`, and starts or restarts it. Inspect the result:

```sh
sudo cat /etc/rgbdns/tinydns.env
sudo systemctl is-enabled rgbdns-tinydns.service
sudo systemctl --no-pager --full status rgbdns-tinydns.service
sudo journalctl -u rgbdns-tinydns.service -b --no-pager
sudo ss -lntup '( sport = :53 )'
```

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

The SOA serial must converge on all four servers.

### 7. Publish updates and keep the service running

For each zone change, edit `/root/cron.sh.data`, increment the serial, and
rerun the same setup command with the complete BuddyNS allow-list:

```sh
sudo editor /root/cron.sh.data
BUDDYNS_AXFR_V4='108.61.224.67/32,116.203.6.3/32,107.191.99.111/32,193.109.120.66/32,5.223.55.119/32,192.184.93.99/32,103.25.56.55/32,216.73.156.203/32,37.143.61.179/32,195.20.17.193/32,45.77.29.133/32,116.203.0.64/32,167.88.161.228/32,199.195.249.208/32,104.244.78.122/32'
sudo rgbdns-setup primary \
  --data /root/cron.sh.data \
  --listen-ip 0.0.0.0 --port 53 \
  --allow-nets "$BUDDYNS_AXFR_V4"
```

Compilation must succeed before systemd restarts the process. The restarted
process loads one consistent zone used for ordinary answers and AXFR. BuddyNS
then refreshes according to its transfer schedule.

`rgbdns-setup` enables the service at boot. The unit runs in the foreground
under systemd, uses `Restart=on-failure`, and recompiles the managed source
before each start. Confirm persistence with:

```sh
sudo systemctl is-enabled rgbdns-tinydns
sudo systemctl restart rgbdns-tinydns
sudo reboot
# reconnect after boot
systemctl is-active rgbdns-tinydns
dig @127.0.0.1 cron.sh SOA +norecurse
```

Monitor at least service state, UDP and TCP queries, SOA serial agreement,
BuddyNS transfer status, disk space, and upcoming package/security updates.
Keep `/root/cron.sh.data` and the exact BuddyNS source list in configuration
management or encrypted backup. A DNS secondary improves serving availability;
it is not a backup of the editable primary source.

## Configure a secondary nameserver

The packaged secondary workflow manages one complete zone per host instance.
It fetches the primary over DNS TCP, verifies response identity, authority,
question, record bounds, zone confinement, and matching SOA bookends, then
atomically installs the new tinydns source and CDB.

Configure the secondary:

```sh
sudo rgbdns-setup secondary \
  --zone example.net \
  --primary 192.0.2.54 \
  --listen-ip 198.51.100.10
```

If the primary uses a nonstandard transfer port:

```sh
sudo rgbdns-setup secondary \
  --zone example.net \
  --primary 192.0.2.53:5354 \
  --listen-ip 127.0.0.1 --port 5353
```

Setup writes `/etc/rgbdns/secondary.env`, performs the first transfer, starts
the authoritative service only after that transfer succeeds, and enables
`rgbdns-secondary-sync.timer`. The timer refreshes every five minutes with a
small randomized delay. Failed transfers leave the last successfully compiled
zone active. A successful transfer atomically replaces `data`, compiles
`data.cdb`, and restarts tinydns.

Run or inspect synchronization manually:

```sh
sudo systemctl start rgbdns-secondary-sync.service
systemctl list-timers rgbdns-secondary-sync.timer
journalctl -u rgbdns-secondary-sync.service
```

Change the interval with a systemd drop-in:

```sh
sudo systemctl edit rgbdns-secondary-sync.timer
```

```ini
[Timer]
OnUnitActiveSec=
OnUnitActiveSec=15min
RandomizedDelaySec=1min
```

Then run:

```sh
sudo systemctl daemon-reload
sudo systemctl restart rgbdns-secondary-sync.timer
```

This is periodic AXFR, not SOA-driven NOTIFY/IXFR. Choose an interval that fits
the zone's change rate and the primary's transfer budget.

## Verify service behavior

Inspect units and logs:

```sh
systemctl status rgbdns-tinydns
journalctl -u rgbdns-tinydns
ss -lntup | grep ':53'
```

Query UDP, TCP, authority, and negative behavior:

```sh
dig @192.0.2.53 example.net SOA +norecurse
dig @192.0.2.53 www.example.net A +norecurse
dig @192.0.2.53 www.example.net A +tcp +norecurse
dig @192.0.2.53 absent.example.net A +norecurse
```

Check that responses carry `aa`, that absent names return NXDOMAIN with the
zone SOA, and that TCP and UDP agree.

## Upgrades, removal, and troubleshooting

Package upgrades preserve `/etc/rgbdns/tinydns.env` as a conffile and do not
automatically enable a service. Managed zone data and optional role files live
outside the package payload. Removing the package stops its units but preserves
configuration and zone state; purge or remove those files explicitly only
after taking a backup.

Common failures:

- `Address already in use`: another resolver or rgbdns unit owns the same
  address and port. Remember that tinydns listens on UDP and TCP.
- `Permission denied` while binding: inspect the unit's capability settings
  and any local systemd override.
- `AXFR server returned an error`: verify the zone name, SOA, primary endpoint,
  source-address allow-list, routing, and TCP firewall.
- `fatal` while compiling: run `tinydns-data` in the state directory and
  correct the reported data-file syntax.
- secondary retains old data: inspect the sync journal. Failed validation never
  replaces the active zone.

Use `systemctl cat UNIT` to include local overrides when diagnosing a host.
