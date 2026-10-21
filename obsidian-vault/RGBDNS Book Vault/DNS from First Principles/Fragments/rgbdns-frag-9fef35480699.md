---
type: "code-fragment"
fragment_id: "rgbdns-frag-9fef35480699"
source_path: "RGBDNS_LETS.md"
code_note: "DNS from First Principles/Code/RGBDNS_LETS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Packaging and service integration"
kind: "heading"
start_line: 249
end_line: 262
---

# Packaging and service integration

- Fragment ID: `rgbdns-frag-9fef35480699`
- Source file: [[DNS from First Principles/Code/RGBDNS_LETS.md.source|RGBDNS_LETS.md]]
- Lines: 249-262
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-9fef35480699", "codeNote": "DNS from First Principles/Code/RGBDNS_LETS.md.source", "sourcePath": "RGBDNS_LETS.md", "startLine": 249, "endLine": 262}
```

## Excerpt

<span id="rgbdns-frag-9fef35480699" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9fef35480699: heading Packaging and service integration

```markdown
## Packaging and service integration

Debian and RPM packages install:

- `/etc/rgbdns/acme-update.conf` as a preserved, mode-0640 example conffile;
- the `rgbdns-acme` binary and manual page;
- updated `tinydns`, `rgbdns-setup`, and import manual pages;
- service defaults with ACME disabled;
- tmpfiles/setup rules for state ownership where required.

`rgbdns-setup --acme-update-config PATH` enables the feature on a primary and
rejects it for a secondary. Package upgrades preserve operator configuration.
Removal does not delete durable ACME state or credentials.

```
