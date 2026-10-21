---
type: "code-fragment"
fragment_id: "rgbdns-frag-21ac1228f795"
source_path: "AGENTS.md"
code_note: "DNS from First Principles/Code/AGENTS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Deployment and packaging invariants"
kind: "heading"
start_line: 56
end_line: 78
---

# Deployment and packaging invariants

- Fragment ID: `rgbdns-frag-21ac1228f795`
- Source file: [[DNS from First Principles/Code/AGENTS.md.source|AGENTS.md]]
- Lines: 56-78
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-21ac1228f795", "codeNote": "DNS from First Principles/Code/AGENTS.md.source", "sourcePath": "AGENTS.md", "startLine": 56, "endLine": 78}
```

## Excerpt

<span id="rgbdns-frag-21ac1228f795" class="rgbdns-fragment-target"></span>
### rgbdns-frag-21ac1228f795: heading Deployment and packaging invariants

```markdown
## Deployment and packaging invariants

- Production zone data and deployment workflows live in the private
  `alexy/cronsh` repository. Wishfully lives in the private
  `querygraph/wishfully` repository. Do not reintroduce either into rgbdns.

- Debian configuration under `/etc/rgbdns` is a conffile and must preserve
  operator changes during upgrade.
- RPM configuration uses `%config(noreplace)` and must preserve operator
  changes during upgrade.
- Systemd units use `RuntimeDirectory=rgbdns`; do not assume `/run/rgbdns`
  exists while the one-shot secondary service is inactive.
- Primary data is imported atomically from `rgbdns.data`.
- Secondary zone lists are imported atomically from `rgbdns.zones`, one zone
  per line. Per-zone synchronization failures retain last-known-good data and
  must not prevent healthy zones from updating.
- The known deployment uses:
  - `a.ns.cron.sh`, public `52.10.53.234`, as the Debian primary;
  - primary VPC address `172.31.60.189` for secondary AXFR;
  - `b.ns.cron.sh`, public `52.38.177.160`, as the openSUSE secondary.
- AXFR allow-lists contain the actual source addresses of secondaries. BuddyNS
  uses public addresses; the EC2 peer should use the VPC path.

```
