---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Running rgbdns under supervision

## The service contract

rgbdns daemons run in the foreground, emit diagnostics to standard error, take
configuration from files and environment, and terminate on fatal startup
errors. That is the portable contract a supervisor needs. The generated
djbdns-style directories additionally provide `run` and `log/run` programs,
but the daemon binaries do not require a particular supervisor.

The classic daemontools control plane is:

```text
supervise service/       keep one process running
svc -u service/          bring it up
svc -d service/          bring it down
svc -t service/          send TERM
svstat service/          inspect status
```

No modern replacement is universally best. Choose according to the host and
the migration boundary.

## Recommendations

### Existing Linux host: systemd

Use systemd when the machine already boots and manages services with systemd.
It supplies dependency ordering, restart policy, socket and readiness models,
resource controls, credential and filesystem sandboxing, a unified journal,
and distribution-native administration. Avoid wrapping an rgbdns daemon in a
second nested supervisor.

A minimal authoritative unit is:

```ini
[Unit]
Description=rgbdns authoritative DNS
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=dns
Group=dns
Environment=IP=192.0.2.53
Environment=PORT=53
Environment=DATA=/etc/rgbdns/data.cdb
ExecStart=/usr/local/bin/tinydns
Restart=on-failure
RestartSec=1s
NoNewPrivileges=yes
ProtectSystem=strict
ProtectHome=yes
PrivateTmp=yes
ReadOnlyPaths=/etc/rgbdns
AmbientCapabilities=CAP_NET_BIND_SERVICE
CapabilityBoundingSet=CAP_NET_BIND_SERVICE

[Install]
WantedBy=multi-user.target
```

Prefer a socket above 1024 or a narrowly bounded bind capability over running
the daemon as root. Test hardening settings on the target distribution because
name-service libraries and trust-anchor paths may require additional read-only
access.

Command mapping:

| daemontools | systemd |
|---|---|
| `svc -u service` | `systemctl start service` |
| `svc -d service` | `systemctl stop service` |
| `svc -t service` | `systemctl kill --signal=TERM service` |
| `svc -h service` | `systemctl kill --signal=HUP service` |
| `svstat service` | `systemctl status service` |
| `multilog` output | journal, or explicit file logging policy |

### Closest service-directory migration: runit

Use runit when you want the smallest conceptual migration from daemontools.
It uses a service directory with a `run` program, keeps the supervised process
in the foreground, has a companion log service, and exposes the compact `sv`
control command. Existing rgbdns generated `run` scripts are close to the
required shape; adjust the directory layout and enablement symlink for the
distribution.

| daemontools | runit |
|---|---|
| `svc -u service` | `sv up service` |
| `svc -d service` | `sv down service` |
| `svc -t service` | `sv term service` |
| `svstat service` | `sv status service` |

Choose runit for minimal hosts, appliances, or migrations where preserving the
service-directory model matters more than rich dependency and sandbox policy.

### Strong supervision composition: s6 and s6-rc

Use s6 when precise process supervision, reliable readiness, and composable
small tools are primary requirements. Its `s6-supervise` and `s6-svc` are close
in spirit to `supervise` and `svc`; `s6-rc` adds declared dependencies and
transactional service-state changes. The ecosystem is particularly effective
in carefully constructed containers and small systems, but its compilation
and directory conventions make migration more involved than runit.

| daemontools | s6 |
|---|---|
| `supervise service` | `s6-supervise service` |
| `svc -u service` | `s6-svc -u service` |
| `svc -d service` | `s6-svc -d service` |
| `svc -t service` | `s6-svc -t service` |
| `svstat service` | `s6-svstat service` |

Choose s6/s6-rc when the team is willing to own its service database and wants
more rigorous dependency transitions than ad hoc shell orchestration.

### OpenRC and container orchestrators

On an OpenRC-based distribution, use the native init integration unless there
is a deliberate reason to introduce another supervision tree. OpenRC service
scripts can use its supervisor support while retaining distribution-standard
boot ordering and administration.

In Kubernetes or a similar orchestrator, run one foreground rgbdns daemon per
container and let the platform own restart, health, resource limits, log
collection, and rollout. Use a Deployment for `tinydns` or `dnscache`, a
Service for stable network reachability, readiness/liveness probes that test
the intended DNS role, ConfigMaps or mounted immutable CDBs for public data,
and Secrets for sensitive material. Do not put systemd, daemontools, and the
orchestrator around the same single process.

An s6-based container is reasonable only when one image intentionally contains
several cooperating long-lived processes and that tradeoff is explicit.

## A practical selection rule

Use this order:

1. Follow the host’s native manager: systemd on systemd hosts, OpenRC on
   OpenRC hosts.
2. For a direct service-directory replacement, choose runit.
3. For a designed supervision graph or multi-process container, choose
   s6/s6-rc.
4. In an orchestrated single-process container, use the orchestrator.

The least risky migration preserves one owner for restart policy and logs.
Running two supervisors creates ambiguous signal paths, duplicate restarts,
and status commands that disagree.

## Generated Code Fragment Index

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-dda2855f47d7", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-dda2855f47d7: enum Service", "sourcePath": "src/conf.rs", "startLine": 11, "endLine": 19}
```

```rgbdns-fragment
{"id": "rgbdns-frag-e9018c070678", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-e9018c070678: fn configure", "sourcePath": "src/conf.rs", "startLine": 20, "endLine": 100}
```

```rgbdns-fragment
{"id": "rgbdns-frag-47c3cd5b1372", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-47c3cd5b1372: fn configure_tinydns", "sourcePath": "src/conf.rs", "startLine": 143, "endLine": 176}
```

```rgbdns-fragment
{"id": "rgbdns-frag-950f5158793d", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-950f5158793d: fn configure_dnscache", "sourcePath": "src/conf.rs", "startLine": 177, "endLine": 191}
```

```rgbdns-fragment
{"id": "rgbdns-frag-f23762f8b911", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-f23762f8b911: fn make_log", "sourcePath": "src/conf.rs", "startLine": 192, "endLine": 207}
```

```rgbdns-fragment
{"id": "rgbdns-frag-40627baa9dd5", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-40627baa9dd5: fn run_script", "sourcePath": "src/conf.rs", "startLine": 208, "endLine": 222}
```

```rgbdns-fragment
{"id": "rgbdns-frag-ca6aa7f85b39", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-ca6aa7f85b39: fn executable", "sourcePath": "src/conf.rs", "startLine": 223, "endLine": 230}
```

```rgbdns-fragment
{"id": "rgbdns-frag-71ec2b6bd4ef", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-71ec2b6bd4ef: fn shell_quote", "sourcePath": "src/conf.rs", "startLine": 231, "endLine": 234}
```

```rgbdns-fragment
{"id": "rgbdns-frag-b0dca80a842d", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-b0dca80a842d: fn write_file", "sourcePath": "src/conf.rs", "startLine": 235, "endLine": 247}
```

```rgbdns-fragment
{"id": "rgbdns-frag-b347d98cd7ff", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-b347d98cd7ff: mod tests", "sourcePath": "src/conf.rs", "startLine": 248, "endLine": 251}
```

```rgbdns-fragment
{"id": "rgbdns-frag-33e30225780d", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-33e30225780d: fn directory", "sourcePath": "src/conf.rs", "startLine": 252, "endLine": 263}
```

```rgbdns-fragment
{"id": "rgbdns-frag-ba9506807310", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-ba9506807310: fn creates_tinydns_service_tree_without_overwriting", "sourcePath": "src/conf.rs", "startLine": 264, "endLine": 285}
```
