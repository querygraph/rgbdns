---
type: "code-fragment"
fragment_id: "rgbdns-frag-533b9c153533"
source_path: "docs/DEBIAN.md"
code_note: "DNS from First Principles/Code/docs/DEBIAN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Complete deployment cron.sh primary with BuddyNS secondaries"
kind: "heading"
start_line: 184
end_line: 264
---

# Complete deployment cron.sh primary with BuddyNS secondaries

- Fragment ID: `rgbdns-frag-533b9c153533`
- Source file: [[DNS from First Principles/Code/docs/DEBIAN.md.source|docs/DEBIAN.md]]
- Lines: 184-264
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-533b9c153533", "codeNote": "DNS from First Principles/Code/docs/DEBIAN.md.source", "heading": "rgbdns-frag-533b9c153533: heading Complete deployment cron.sh primary with BuddyNS secondaries", "sourcePath": "docs/DEBIAN.md", "startLine": 184, "endLine": 264}
```

## Excerpt

<span id="rgbdns-frag-533b9c153533" class="rgbdns-fragment-target"></span>
### rgbdns-frag-533b9c153533: heading Complete deployment cron.sh primary with BuddyNS secondaries

```markdown
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
```
