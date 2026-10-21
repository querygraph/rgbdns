---
type: "code-fragment"
fragment_id: "rgbdns-frag-d2e6160cf4aa"
source_path: "docs/OPENSUSE.md"
code_note: "DNS from First Principles/Code/docs/OPENSUSE.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Installing rgbdns on openSUSE Leap 16.0 on AWS EC2"
kind: "heading"
start_line: 1
end_line: 16
---

# Installing rgbdns on openSUSE Leap 16.0 on AWS EC2

- Fragment ID: `rgbdns-frag-d2e6160cf4aa`
- Source file: [[DNS from First Principles/Code/docs/OPENSUSE.md.source|docs/OPENSUSE.md]]
- Lines: 1-16
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-d2e6160cf4aa", "codeNote": "DNS from First Principles/Code/docs/OPENSUSE.md.source", "sourcePath": "docs/OPENSUSE.md", "startLine": 1, "endLine": 16}
```

## Excerpt

<span id="rgbdns-frag-d2e6160cf4aa" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d2e6160cf4aa: heading Installing rgbdns on openSUSE Leap 16.0 on AWS EC2

```markdown
# Installing rgbdns on openSUSE Leap 16.0 on AWS EC2

For the complete two-host `fieldnotes.es` example, including the Debian
primary and atomic `rgbdns.data`/`rgbdns.zones` pickup, see
[`RGBDNS_SETUP.md`](RGBDNS_SETUP.md).

This guide targets the current openSUSE AWS Marketplace offering:
**openSUSE Leap 16.0 (x86_64), Marketplace version v20260213**. AWS Marketplace
AMI IDs are regional and may change when the publisher updates the product, so
select the current version from the Marketplace product rather than copying an
AMI ID from another region.

The RPM installs the complete command suite, hardened systemd units, a
dedicated unprivileged account, and `rgbdns-setup`. Installing the package does
not enable or start DNS. Configuration is an explicit administrator action.

```
