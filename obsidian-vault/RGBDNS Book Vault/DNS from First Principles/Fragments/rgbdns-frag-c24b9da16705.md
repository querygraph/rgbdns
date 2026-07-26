---
type: "code-fragment"
fragment_id: "rgbdns-frag-c24b9da16705"
source_path: "README.md"
code_note: "DNS from First Principles/Code/README.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Debian and systemd"
kind: "heading"
start_line: 57
end_line: 74
---

# Debian and systemd

- Fragment ID: `rgbdns-frag-c24b9da16705`
- Source file: [[DNS from First Principles/Code/README.md.source|README.md]]
- Lines: 57-74
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-c24b9da16705", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-c24b9da16705: heading Debian and systemd", "sourcePath": "README.md", "startLine": 57, "endLine": 74}
```

## Excerpt

<span id="rgbdns-frag-c24b9da16705" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c24b9da16705: heading Debian and systemd

```markdown
## Debian and systemd

The repository includes native Debian packaging, hardened systemd services, and
an idempotent `rgbdns-setup` command for primary and secondary authoritative
servers. See [`docs/DEBIAN.md`](docs/DEBIAN.md) for package builds, account and
directory layout, tinydns data-file setup, firewalls, AXFR allow-lists, timed
secondary refresh, verification, upgrades, and troubleshooting. It includes a
complete `cron.sh` deployment with `52.10.53.234` as the primary address and
BuddyNS as the secondary network.

On Debian or Ubuntu, build the package with:

```sh
sudo apt install build-essential cargo debhelper rustc
packaging/build-deb.sh
sudo apt install ../rgbdns_0.1.1_$(dpkg --print-architecture).deb
```

```
