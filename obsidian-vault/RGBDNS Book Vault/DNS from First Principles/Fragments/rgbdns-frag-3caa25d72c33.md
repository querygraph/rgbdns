---
type: "code-fragment"
fragment_id: "rgbdns-frag-3caa25d72c33"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "5. Install the RPM on the secondary"
kind: "heading"
start_line: 152
end_line: 180
---

# 5. Install the RPM on the secondary

- Fragment ID: `rgbdns-frag-3caa25d72c33`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 152-180
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-3caa25d72c33", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 152, "endLine": 180}
```

## Excerpt

<span id="rgbdns-frag-3caa25d72c33" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3caa25d72c33: heading 5. Install the RPM on the secondary

````markdown
## 5. Install the RPM on the secondary

Set the actual openSUSE login name:

```sh
SUSE_USER=ec2-user
```

Copy the package:

```sh
scp "$HOME/rgbdns-rpm/RPMS/x86_64/rgbdns-0.6.5-1.x86_64.rpm" \
  "$SUSE_USER"@52.38.177.160:/tmp/
```

On the secondary:

```sh
sudo zypper --non-interactive --no-gpg-checks install \
  /tmp/rgbdns-0.6.5-1.x86_64.rpm
sudo systemctl daemon-reload
rpm -q rgbdns
sudo rpm -V rgbdns
getent passwd rgbdns
```

An upgrade from the earlier `ZONES=` layout migrates the list once to
`/var/lib/rgbdns/tinydns/zones`. New setup writes the canonical file directly.

````
