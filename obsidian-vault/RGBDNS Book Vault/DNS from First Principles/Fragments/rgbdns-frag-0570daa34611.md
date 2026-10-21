---
type: "code-fragment"
fragment_id: "rgbdns-frag-0570daa34611"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "4. Install the Debian package on the primary"
kind: "heading"
start_line: 129
end_line: 151
---

# 4. Install the Debian package on the primary

- Fragment ID: `rgbdns-frag-0570daa34611`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 129-151
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-0570daa34611", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 129, "endLine": 151}
```

## Excerpt

<span id="rgbdns-frag-0570daa34611" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0570daa34611: heading 4. Install the Debian package on the primary

````markdown
## 4. Install the Debian package on the primary

Copy the package:

```sh
scp "$HOME/rgbdns-deb/rgbdns_0.6.5_amd64.deb" \
  bitnami@52.10.53.234:/tmp/
```

On the primary:

```sh
sudo apt install /tmp/rgbdns_0.6.5_amd64.deb
sudo systemctl daemon-reload
dpkg-query -W -f='${Package} ${Version}\n' rgbdns
getent passwd rgbdns
```

During an upgrade, retain the existing `/etc/rgbdns/tinydns.env` when `dpkg`
asks about the locally modified conffile. It contains the deployment-specific
AXFR allow-list. A missing `QUERY_LOG` assignment still means request logging
is enabled, although this walkthrough writes it explicitly.

````
