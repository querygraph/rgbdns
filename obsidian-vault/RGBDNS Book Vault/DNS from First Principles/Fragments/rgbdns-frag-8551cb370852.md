---
type: "code-fragment"
fragment_id: "rgbdns-frag-8551cb370852"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "14. Publish later secondary-list changes"
kind: "heading"
start_line: 576
end_line: 609
---

# 14. Publish later secondary-list changes

- Fragment ID: `rgbdns-frag-8551cb370852`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 576-609
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-8551cb370852", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 576, "endLine": 609}
```

## Excerpt

<span id="rgbdns-frag-8551cb370852" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8551cb370852: heading 14. Publish later secondary-list changes

````markdown
## 14. Publish later secondary-list changes

Edit `rgbdns.zones`, one zone per line. Blank lines and leading `#` comment
lines are accepted. Publish atomically:

```sh
scp rgbdns.zones \
  "$SUSE_USER"@52.38.177.160:/var/lib/rgbdns/incoming/rgbdns.zones.new &&
ssh "$SUSE_USER"@52.38.177.160 \
  'mv /var/lib/rgbdns/incoming/rgbdns.zones.new \
      /var/lib/rgbdns/incoming/rgbdns.zones'
```

The importer rejects symlinks, unexpected ownership, malformed names, and an
empty list. It lowercases names, removes duplicates, atomically installs
`/var/lib/rgbdns/tinydns/zones`, and starts synchronization.

Inspect:

```sh
ssh "$SUSE_USER"@52.38.177.160 \
  'sudo journalctl -u rgbdns-zones-import.service \
     -u rgbdns-secondary-sync.service -n 100 --no-pager'
```

Before adding a zone, confirm the primary already serves and transfers it:

```sh
dig @172.31.60.189 new-zone.example SOA +norecurse
dig @172.31.60.189 new-zone.example AXFR
```

Run those commands from the secondary or another authorized VPC source.

````
