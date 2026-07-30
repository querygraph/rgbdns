# rgbdns Debian Package Review 1

Reviewed against `querygraph/rgbdns` master commit
[`067c52b`](https://github.com/querygraph/rgbdns/commit/067c52b88917c31332bca50803d442c27b9ed4df).

## Summary

The new Debian setup is substantially more robust. It provides hardened
systemd services, primary and secondary configuration automation, package
verification, and a genuine AXFR workflow.

It should not yet be deployed unchanged as the public rgbdns primary feeding
BuddyNS from the single public address `52.10.53.234`. Three production issues
should be addressed first.

## Findings

### 1. Critical: AXFR cannot share the only public address on port 53

`tinydns` binds both UDP and TCP. `axfrdns` separately binds TCP. Consequently,
the two processes cannot both bind:

```text
52.10.53.234:53
```

The setup script correctly detects and rejects this collision. For the current
EC2 deployment, however, BuddyNS cannot AXFR from the standard DNS endpoint
unless one of these approaches is used:

- Add a second private IP and a second public Elastic IP dedicated to AXFR.
- Change rgbdns so that one TCP listener routes ordinary DNS queries to the
  authoritative handler and AXFR queries to the AXFR handler.

The combined TCP listener is the preferred solution. It preserves one public
IP and standard TCP port 53.

### 2. High: rerunning `rgbdns-setup primary` does not reload running services

The setup script replaces the data and environment files and then runs:

```sh
systemctl enable --now rgbdns-tinydns.service
```

If the service is already running, `enable --now` does not restart it. The
existing process continues serving its old in-memory zone and old environment.
The same problem applies to `rgbdns-axfrdns`.

The command is filesystem-idempotent but not operationally idempotent. It
should enable the units and then explicitly restart the configured services.

### 3. High: ordinary primary zone updates leave AXFR stale

The documented primary update procedure is:

```sh
sudo -u rgbdns /usr/lib/rgbdns/compile-zone
sudo systemctl restart rgbdns-tinydns
```

`axfrdns` also loads `data.cdb` into memory when it starts. Restarting only
tinydns therefore creates inconsistent views:

- Public queries receive the new zone.
- BuddyNS AXFR receives the old zone.

Both services must be restarted after successful compilation:

```sh
sudo systemctl restart rgbdns-tinydns rgbdns-axfrdns
```

Ideally, the package should provide an `rgbdns-reload` helper that first
compiles successfully and then restarts every active primary service.

## Additional issues

- Switching from primary to secondary does not disable a previously enabled
  AXFR service.
- Switching from secondary to primary does not disable the secondary
  synchronization timer.
- Omitting AXFR options during a later primary setup leaves the old
  `/etc/rgbdns/axfrdns.env` and potentially its running service behind.
- The package version remains `0.1.0`. Rebuilding changed packages under the
  same version makes upgrades ambiguous. Debian revisions such as `0.1.0-1`
  and `0.1.0-2` should be used.
- AXFR authentication uses source-address allow-listing and does not implement
  TSIG. This is acceptable for BuddyNS when both AWS and rgbdns restrict
  transfers to the exact BuddyNS transfer addresses.

## Strengths

- Dedicated non-login `rgbdns` service account.
- Hardened systemd units with only `CAP_NET_BIND_SERVICE`.
- Primary and secondary setup automation.
- Atomic secondary synchronization with locking.
- Failed transfers preserve the last valid compiled zone.
- AXFR structural validation and bounded resource usage.
- Five-minute secondary synchronization timer.
- Automatic discovery and packaging of every Cargo binary.
- Lintian, clean-container installation, and packaged-binary verification in
  GitHub Actions.
- Installation alone deliberately does not start a DNS service.

## Recommendation

The package machinery is strong. Before production deployment on the current
machine:

1. Combine authoritative TCP and AXFR handling on `52.10.53.234:53`.
2. Make `rgbdns-setup` restart already-running services after configuration
   changes.
3. Ensure zone reloads restart both tinydns and axfrdns.
4. Cleanly disable stale units and configuration when changing server roles.
5. Adopt Debian package revisions for every packaging release.

After those changes, the package will support the intended architecture:
rgbdns as the public primary and canonical zone source, with BuddyNS providing
secondary authoritative service through AXFR.
