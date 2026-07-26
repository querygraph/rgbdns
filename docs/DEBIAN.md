# Installing rgbdns on Debian with systemd

The Debian package installs the complete rgbdns command suite, four systemd
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
sudo apt install ../rgbdns_0.1.0_$(dpkg --print-architecture).deb
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
- systemd units for authoritative DNS, AXFR serving, and secondary refresh.

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

`tinydns` already owns both UDP and TCP on its configured endpoint. The
separate `axfrdns` process therefore needs either:

- another local IP on TCP port 53; or
- a different TCP port, normally only for private networks, NAT, or testing.

AXFR has no TSIG implementation in this release. Its security boundary is the
explicit source-address CIDR allow-list, so use narrow secondary addresses and
enforce the same policy in the host and perimeter firewalls.

For a primary whose ordinary DNS address is `192.0.2.53`, serve transfers on a
second address and allow two secondary hosts:

```sh
sudo rgbdns-setup primary \
  --data /root/example.net.data \
  --listen-ip 192.0.2.53 \
  --axfr-listen-ip 192.0.2.54 \
  --allow-nets 198.51.100.10/32,2001:db8:100::10/128
```

For a lab using one address, choose a non-conflicting port:

```sh
sudo rgbdns-setup primary \
  --data /root/example.net.data \
  --listen-ip 127.0.0.1 --port 5353 \
  --axfr-listen-ip 127.0.0.1 --axfr-port 5354 \
  --allow-nets 127.0.0.1/32
```

The generated `/etc/rgbdns/axfrdns.env` contains `IP`, `PORT`, `DATA`, and
`ALLOW_NETS`. Protect that file as configuration even though the allow-list is
not a secret.

For nftables, a narrowly scoped IPv4 rule resembles:

```nft
ip saddr { 198.51.100.10, 198.51.100.11 } \
  ip daddr 192.0.2.54 tcp dport 53 accept
```

Permit both UDP and TCP 53 to the public authoritative endpoint, but only TCP
to the AXFR endpoint. Adjust the example to the host's existing table and chain
rather than pasting it blindly.

Test a transfer from an allowed secondary:

```sh
dig +tcp AXFR example.net @192.0.2.54
# or:
axfr-get example.net 192.0.2.54 /tmp/example.data /tmp/example.data.tmp
```

An unlisted source is disconnected without receiving zone contents.

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
systemctl status rgbdns-tinydns rgbdns-axfrdns
journalctl -u rgbdns-tinydns -u rgbdns-axfrdns
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
