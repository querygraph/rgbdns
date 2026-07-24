---
type: "code-fragment"
fragment_id: "rgbdns-frag-0d9042da275d"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Recommendations"
kind: "heading"
start_line: 778
end_line: 858
---

# Recommendations

- Fragment ID: `rgbdns-frag-0d9042da275d`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 778-858
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-0d9042da275d", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-0d9042da275d: heading Recommendations", "sourcePath": "docs/book/rgbdns.md", "startLine": 778, "endLine": 858}
```

## Excerpt

<span id="rgbdns-frag-0d9042da275d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0d9042da275d: heading Recommendations

```markdown
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
```
