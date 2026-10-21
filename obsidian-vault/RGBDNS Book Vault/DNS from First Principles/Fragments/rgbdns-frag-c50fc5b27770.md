---
type: "code-fragment"
fragment_id: "rgbdns-frag-c50fc5b27770"
source_path: "README.md"
code_note: "DNS from First Principles/Code/README.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Linux packages and systemd"
kind: "heading"
start_line: 112
end_line: 127
---

# Linux packages and systemd

- Fragment ID: `rgbdns-frag-c50fc5b27770`
- Source file: [[DNS from First Principles/Code/README.md.source|README.md]]
- Lines: 112-127
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-c50fc5b27770", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 112, "endLine": 127}
```

## Excerpt

<span id="rgbdns-frag-c50fc5b27770" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c50fc5b27770: heading Linux packages and systemd

```markdown
## Linux packages and systemd

The repository includes native Debian and openSUSE RPM packaging, hardened systemd services, and
an idempotent `rgbdns-setup` command for primary and secondary authoritative
servers. See [`docs/DEBIAN.md`](docs/DEBIAN.md) for package builds, account and
directory layout, tinydns data-file setup, firewalls, AXFR allow-lists, timed
secondary refresh, verification, upgrades, and troubleshooting. It includes a
complete example based on the cron.sh deployment topology. Live zone state is
kept in the private `alexy/cronsh` repository.

Primaries watch `rgbdns.data`, and secondaries watch a canonical
one-zone-per-line `/var/lib/rgbdns/incoming/rgbdns.zones` in managed state.
Publish either file through a `.new` name followed by a remote rename. rgbdns
validates and compiles primary data before activation, and validates secondary
lists before starting AXFR refresh.

```
