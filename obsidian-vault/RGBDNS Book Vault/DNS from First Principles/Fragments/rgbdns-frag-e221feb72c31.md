---
type: "code-fragment"
fragment_id: "rgbdns-frag-e221feb72c31"
source_path: "docs/work-review-1.md"
code_note: "DNS from First Principles/Code/docs/work-review-1.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Findings"
kind: "heading"
start_line: 16
end_line: 76
---

# Findings

- Fragment ID: `rgbdns-frag-e221feb72c31`
- Source file: [[DNS from First Principles/Code/docs/work-review-1.md.source|docs/work-review-1.md]]
- Lines: 16-76
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-e221feb72c31", "codeNote": "DNS from First Principles/Code/docs/work-review-1.md.source", "sourcePath": "docs/work-review-1.md", "startLine": 16, "endLine": 76}
```

## Excerpt

<span id="rgbdns-frag-e221feb72c31" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e221feb72c31: heading Findings

````markdown
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

````
