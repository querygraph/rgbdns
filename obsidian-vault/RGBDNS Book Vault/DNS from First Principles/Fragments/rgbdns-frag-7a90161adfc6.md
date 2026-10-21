---
type: "code-fragment"
fragment_id: "rgbdns-frag-7a90161adfc6"
source_path: "docs/DEBIAN.md"
code_note: "DNS from First Principles/Code/docs/DEBIAN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Installing rgbdns on Debian with systemd"
kind: "heading"
start_line: 1
end_line: 11
---

# Installing rgbdns on Debian with systemd

- Fragment ID: `rgbdns-frag-7a90161adfc6`
- Source file: [[DNS from First Principles/Code/docs/DEBIAN.md.source|docs/DEBIAN.md]]
- Lines: 1-11
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-7a90161adfc6", "codeNote": "DNS from First Principles/Code/docs/DEBIAN.md.source", "sourcePath": "docs/DEBIAN.md", "startLine": 1, "endLine": 11}
```

## Excerpt

<span id="rgbdns-frag-7a90161adfc6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7a90161adfc6: heading Installing rgbdns on Debian with systemd

```markdown
# Installing rgbdns on Debian with systemd

For the complete two-host `fieldnotes.es` example, including the openSUSE
secondary and atomic `rgbdns.data`/`rgbdns.zones` pickup, see
[`RGBDNS_SETUP.md`](RGBDNS_SETUP.md).

The Debian package installs the complete rgbdns command suite, hardened systemd
units, a dedicated unprivileged account, and `rgbdns-setup`. Installation alone
does not publish a DNS service. The administrator must choose a primary or
secondary role and provide the corresponding data.

```
