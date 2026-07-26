---
type: "code-fragment"
fragment_id: "rgbdns-frag-c5ffd76a5d56"
source_path: "docs/DEBIAN.md"
code_note: "DNS from First Principles/Code/docs/DEBIAN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Verify service behavior"
kind: "heading"
start_line: 518
end_line: 539
---

# Verify service behavior

- Fragment ID: `rgbdns-frag-c5ffd76a5d56`
- Source file: [[DNS from First Principles/Code/docs/DEBIAN.md.source|docs/DEBIAN.md]]
- Lines: 518-539
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-c5ffd76a5d56", "codeNote": "DNS from First Principles/Code/docs/DEBIAN.md.source", "heading": "rgbdns-frag-c5ffd76a5d56: heading Verify service behavior", "sourcePath": "docs/DEBIAN.md", "startLine": 518, "endLine": 539}
```

## Excerpt

<span id="rgbdns-frag-c5ffd76a5d56" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c5ffd76a5d56: heading Verify service behavior

```markdown
## Verify service behavior

Inspect units and logs:

```sh
systemctl status rgbdns-tinydns
journalctl -u rgbdns-tinydns
ss -lntup | grep ':53'
```

Query UDP, TCP, authority, and negative behavior:

```sh
dig @192.0.2.53 example.net SOA +norecurse
dig @192.0.2.53 www.example.net A +norecurse
dig @192.0.2.53 www.example.net A +tcp +norecurse
dig @192.0.2.53 absent.example.net A +norecurse
```

Check that responses carry `aa`, that absent names return NXDOMAIN with the
zone SOA, and that TCP and UDP agree.

```
