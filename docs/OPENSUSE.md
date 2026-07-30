# Installing rgbdns on openSUSE Leap 16.0 on AWS EC2

This guide targets the current openSUSE AWS Marketplace offering:
**openSUSE Leap 16.0 (x86_64), Marketplace version v20260213**. AWS Marketplace
AMI IDs are regional and may change when the publisher updates the product, so
select the current version from the Marketplace product rather than copying an
AMI ID from another region.

The RPM installs the complete command suite, three hardened systemd units, a
dedicated unprivileged account, and `rgbdns-setup`. Installing the package does
not enable or start DNS. Configuration is an explicit administrator action.

## Launch the current Marketplace image

1. In the EC2 console, choose **Launch instance**.
2. Under **Application and OS Images**, choose **Browse more AMIs**, then
   **AWS Marketplace AMIs**.
3. Select the openSUSE-published **openSUSE Leap** product and its current
   **Leap 16.0 (x86_64)** delivery option.
4. Choose an x86_64 instance with enough memory to compile Rust. A small
   production DNS instance can run in modest memory, but building is more
   comfortable on a separate build host or a temporary instance with at least
   4 GiB.
5. Require IMDSv2, use an encrypted EBS volume, attach the SSH key, and assign
   the instance an Elastic IP if it will be an Internet authority.
6. Allow SSH only from administrator addresses. Add public UDP 53 and TCP 53.
   Ordinary DNS requires both; AXFR shares the TCP listener and is restricted
   separately inside rgbdns.

The Marketplace image's SSH user is `ec2-user`:

```sh
ssh -i ~/.ssh/KEY.pem ec2-user@PUBLIC_ADDRESS
sudo zypper refresh
sudo zypper --non-interactive update
sudo zypper --non-interactive install bind-utils
cat /etc/os-release
uname -m
```

Reboot if the update installs a new kernel, then reconnect. The examples below
assume `x86_64`. Build an architecture-matching RPM for any other target.

## Build the RPM

On a Leap 16.0 x86_64 build host:

```sh
sudo zypper --non-interactive install \
  git cargo rust python3 rpm-build systemd-rpm-macros
git clone https://github.com/querygraph/rgbdns.git
cd rgbdns
packaging/build-rpm.sh
rpm -qip dist/rpmbuild/RPMS/x86_64/rgbdns-0.2.2-1.x86_64.rpm
rpm -qlp dist/rpmbuild/RPMS/x86_64/rgbdns-0.2.2-1.x86_64.rpm
```

`build-rpm.sh` creates a clean rpmbuild tree under `dist/rpmbuild`, archives
the tracked and non-ignored working tree, and builds both source and binary RPMs. The spec uses
`cargo build --release --locked --bins` and runs the complete release test
suite. It discovers binary targets through Cargo metadata rather than keeping
a second program list.

The `Build openSUSE RPM` GitHub Actions workflow performs the same build in an
openSUSE Leap 16.0 container, installs and verifies the RPM, and publishes the
binary and source packages as the
`rgbdns-opensuse-leap16-x86_64` workflow artifact:

```sh
gh workflow run build-rpm.yml --ref master
gh run watch
gh run download RUN_ID \
  -n rgbdns-opensuse-leap16-x86_64 \
  -D dist/cloud-rpm
```

Inspect an artifact before installation:

```sh
rpm -K dist/cloud-rpm/rgbdns-0.2.2-1.x86_64.rpm
rpm -qip dist/cloud-rpm/rgbdns-0.2.2-1.x86_64.rpm
rpm -qlp dist/cloud-rpm/rgbdns-0.2.2-1.x86_64.rpm
```

The development RPM is not repository-signed. `rpm -K` still verifies the
embedded payload digest, but production distribution should place it in a
signed, access-controlled RPM repository.

## Install and inspect the package

Copy a locally built package to the EC2 instance:

```sh
scp -i ~/.ssh/KEY.pem \
  dist/rpmbuild/RPMS/x86_64/rgbdns-0.2.2-1.x86_64.rpm \
  ec2-user@PUBLIC_ADDRESS:/tmp/
ssh -i ~/.ssh/KEY.pem ec2-user@PUBLIC_ADDRESS
sudo zypper --non-interactive --no-gpg-checks install \
  /tmp/rgbdns-0.2.2-1.x86_64.rpm
rpm -q rgbdns
rpm -V rgbdns
```

Installation creates:

- the non-login `rgbdns` system user and group;
- `/etc/rgbdns`, owned by `root:rgbdns`;
- `/var/lib/rgbdns/tinydns`, owned by `rgbdns:rgbdns`;
- commands under `/usr/bin` and `rgbdns-setup` under `/usr/sbin`;
- helpers under `/usr/lib/rgbdns`;
- systemd units under `/usr/lib/systemd/system`.

The units are installed disabled and stopped. They bind port 53 with
`CAP_NET_BIND_SERVICE` while the server remains the unprivileged `rgbdns` user.
Filesystem and kernel protections constrain the services to their managed
state and required address families.

Check that no other daemon owns port 53:

```sh
sudo ss -lntup '( sport = :53 )'
sudo systemctl status named unbound dnsmasq systemd-resolved 2>/dev/null || true
```

Resolve conflicts deliberately. Do not break the instance's recursive resolver
or `/etc/resolv.conf`; rgbdns is the public authoritative service, not a
replacement for the host's resolver in this setup.

## Configure firewalld and the EC2 security group

The EC2 security group must permit:

- UDP 53 from `0.0.0.0/0`;
- TCP 53 from `0.0.0.0/0`;
- SSH TCP 22 only from trusted administration networks.

If the instance has a routed IPv6 address and the zone publishes it, add the
equivalent IPv6 DNS rules. Do not limit all TCP 53 to the secondaries: normal
clients need TCP fallback. rgbdns applies its transfer allow-list only to AXFR
questions.

If firewalld is already active, enable the predefined DNS service in the
instance's active zone:

```sh
sudo systemctl is-active firewalld
sudo firewall-cmd --get-active-zones
sudo firewall-cmd --permanent --add-service=dns
sudo firewall-cmd --reload
sudo firewall-cmd --list-services
```

If firewalld is not installed or active, preserve that policy choice and make
the equivalent change in the host firewall actually in use. Do not enable a
new remote firewall until its active zone explicitly permits the current SSH
administration path. Keep the AWS security group and host firewall consistent.

Leap 16.0 can run SELinux. Leave it enforcing:

```sh
getenforce 2>/dev/null || true
sudo journalctl -t setroubleshoot --since today 2>/dev/null || true
```

The packaged service uses standard executable, configuration, and state paths.
If a locally customized policy denies access, inspect the audit record and add
a narrow local rule; do not disable SELinux as a workaround.

## Complete deployment: cron.sh primary with BuddyNS

This example uses the EC2 instance behind Elastic IP `52.10.53.234` as
`a.ns.cron.sh`, the editable primary for `cron.sh`. BuddyNS copies the zone by
AXFR and serves it from its secondary network. The guest normally sees its
private address, so rgbdns listens on `0.0.0.0:53` and AWS performs the Elastic
IP translation.

### Create the primary zone

Create the protected canonical source:

```sh
sudo install -m 0600 /dev/null /root/cron.sh.data
sudoedit /root/cron.sh.data
```

Start with:

```text
Zcron.sh:a.ns.cron.sh:hostmaster.cron.sh:2026072601:16384:2048:1048576:2560:3600
&cron.sh:52.10.53.234:a.ns.cron.sh:3600
&cron.sh::uz5x6wcwzfbjs8fkmkuchydn9339lf7xbxdmnp038cmyjlgg9sprr2.free.ns.buddyns.com:3600
&cron.sh::uz5dkwpjfvfwb9rh1qj93mtup0gw65s6j7vqqumch0r9gzlu8qxx39.free.ns.buddyns.com:3600
&cron.sh::uz56xw8h7fw656bpfv84pctjbl9rbzbqrw4rpzdhtvzyltpjdmx0zq.free.ns.buddyns.com:3600
+a.ns.cron.sh:52.10.53.234:3600
```

Use the secondary names assigned in BuddyBoard if they differ. The empty
address fields on BuddyNS `&` records are intentional; their glue belongs to
BuddyNS. Add the application's A, AAAA, MX, TXT, CAA, or other records and
increment the SOA serial on every publication.

### Configure integrated AXFR

BuddyNS requires every address in its current transfer-source inventory to be
allowed. Recheck BuddyNS's published zone-transfer instructions before
deployment and after provider network changes. As recorded on 2026-07-26, the
IPv4 sources were:

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

Represent them as exact `/32` networks and configure the service:

```sh
BUDDYNS_AXFR_V4='108.61.224.67/32,116.203.6.3/32,107.191.99.111/32,193.109.120.66/32,5.223.55.119/32,192.184.93.99/32,103.25.56.55/32,216.73.156.203/32,37.143.61.179/32,195.20.17.193/32,45.77.29.133/32,116.203.0.64/32,167.88.161.228/32,199.195.249.208/32,104.244.78.122/32'
sudo rgbdns-setup primary \
  --data /root/cron.sh.data \
  --listen-ip 0.0.0.0 \
  --port 53 \
  --allow-nets "$BUDDYNS_AXFR_V4"
```

The command validates and copies the source, compiles `data.cdb`, writes
`/etc/rgbdns/tinydns.env`, enables the unit at boot, and starts it. Inspect the
result:

It also watches `rgbdns.data` in the invoking sudo user's home. Publish future
versions through a temporary name and atomic rename:

```sh
scp rgbdns.data a.ns.cron.sh:rgbdns.data.new
ssh a.ns.cron.sh 'mv rgbdns.data.new rgbdns.data'
```

`rgbdns-data.path` checks ownership, rejects symlinks, compiles a staged copy,
and atomically activates it only after successful validation. Use
`--data-drop FILE` and `--data-drop-owner USER` to override the destination.

```sh
sudo cat /etc/rgbdns/tinydns.env
sudo systemctl is-enabled rgbdns-tinydns.service
sudo systemctl --no-pager --full status rgbdns-tinydns.service
sudo journalctl -u rgbdns-tinydns.service -b --no-pager
sudo ss -lntup '( sport = :53 )'
```

### Request logs

The authoritative server emits one original-compatible request record to
stderr for every UDP, TCP, malformed, refused, and AXFR query:

```text
7f000001:e214:0018 + 0001 fieldnotes.es
```

The packaged systemd unit sends these untimestamped records to journald:

```sh
sudo journalctl -fu rgbdns-tinydns.service
```

The same raw stream can instead feed a daemontools `multilog t` logger, which
adds TAI64N timestamps and rotation. Names are escaped and each request is
written as one complete line. Use only one logging sink per service. To
disable request logs intentionally, pass `--query-log 0` to `rgbdns-setup` or
set `QUERY_LOG=0` in `/etc/rgbdns/tinydns.env` and restart the service. Query
logs contain client addresses and requested names, so protect and size their
retention appropriately.

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
  --listen-ip 0.0.0.0 --port 53 \
  --allow-nets "$BUDDYNS_AXFR_V4"
```

Confirm boot persistence:

```sh
sudo systemctl is-enabled rgbdns-tinydns
sudo systemctl restart rgbdns-tinydns
sudo reboot
# Reconnect after boot.
systemctl is-active rgbdns-tinydns
dig @127.0.0.1 cron.sh SOA +norecurse
```

Monitor the systemd unit, public UDP and TCP answers, BuddyNS transfer status,
SOA convergence, disk space, and Leap security updates. Back up
`/root/cron.sh.data` and the transfer-source policy. A secondary DNS network
improves availability but is not a backup of the editable source.

## Configure an rgbdns secondary

The package can instead manage a list of zones transferred from one primary:

```sh
sudo rgbdns-setup secondary \
  --zone example.net \
  --zone example.org \
  --primary 192.0.2.54 \
  --listen-ip 198.51.100.10
```

Setup performs every initial AXFR before starting authority, enables
`rgbdns-tinydns.service` and `rgbdns-secondary-sync.timer`, writes the canonical
one-zone-per-line list to `/etc/rgbdns/zones`, and watches `rgbdns.zones` in
the invoking sudo user's home directory. Use `--zones-drop FILE` and
`--zones-drop-owner USER` to override that destination.

To permit an additional secondary provider to transfer the validated zone
from this secondary, pass its exact AXFR source CIDRs:

```sh
sudo rgbdns-setup secondary \
  --zones "example.net example.org" \
  --primary 192.0.2.54 \
  --listen-ip 198.51.100.10 \
  --allow-nets 203.0.113.10/32,203.0.113.11/32
```

Use the provider's current published transfer-source list. Ordinary UDP and
TCP answers remain public; `--allow-nets` restricts only AXFR.

Publish later zone-list changes atomically:

```text
fieldnotes.es
example.net
example.org
```

```sh
scp rgbdns.zones secondary.example:rgbdns.zones.new
ssh secondary.example 'mv rgbdns.zones.new rgbdns.zones'
```

`rgbdns-zones.path` starts a protected importer after the final rename. The
importer rejects symlinks, unexpected ownership, malformed names, and empty
lists; normalizes and deduplicates valid names; atomically replaces
`/etc/rgbdns/zones`; and starts AXFR synchronization. A failed refresh retains
that zone's last-known-good snapshot while successful zones advance. A newly
listed zone must transfer successfully before the new combined list can be
activated.

Inspect or invoke synchronization:

```sh
sudo systemctl start rgbdns-secondary-sync.service
systemctl list-timers rgbdns-secondary-sync.timer
systemctl status rgbdns-zones.path
journalctl -u rgbdns-zones-import.service \
  -u rgbdns-secondary-sync.service
```

This is periodic AXFR, not NOTIFY/IXFR.

## Upgrade, remove, and troubleshoot

Upgrade a locally supplied RPM with:

```sh
sudo zypper --non-interactive --no-gpg-checks install \
  /tmp/rgbdns-NEW_VERSION.x86_64.rpm
sudo systemctl restart rgbdns-tinydns
rpm -V rgbdns
```

The `%config(noreplace)` environment file is preserved across upgrades. Zone
state and `/root/cron.sh.data` are not package payloads.

Stop and remove the software without deleting configuration or zone state:

```sh
sudo systemctl disable --now \
  rgbdns-secondary-sync.timer rgbdns-tinydns.service
sudo zypper --non-interactive remove rgbdns
```

Back up and explicitly remove `/etc/rgbdns` and `/var/lib/rgbdns` only when
retiring the service.

Common failures:

- `Address already in use`: inspect `ss` and stop or rebind the conflicting
  resolver.
- port 53 permission failure: inspect the unit's capability settings and local
  overrides with `systemctl cat rgbdns-tinydns`.
- AXFR failure: check the zone, current BuddyNS source list, TCP security-group
  and firewalld rules, routing, and `ALLOW_NETS`.
- compilation failure: run `sudo -u rgbdns /usr/lib/rgbdns/compile-zone` and
  correct the reported source line.
- service denial under SELinux: inspect the audit log and label/customize only
  the denied path or operation.
- stale secondary: inspect `journalctl -u rgbdns-secondary-sync.service`;
  rejected transfers never overwrite the last valid data.

Useful diagnostics:

```sh
sudo systemd-analyze verify \
  /usr/lib/systemd/system/rgbdns-tinydns.service
sudo journalctl -u rgbdns-tinydns -b
sudo ss -lntup '( sport = :53 )'
sudo rpm -V rgbdns
```
