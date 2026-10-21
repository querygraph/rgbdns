---
type: "code-fragment"
fragment_id: "rgbdns-frag-388087cb9761"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "+www.fieldnotes.es 192.0.2.80 300"
kind: "heading"
start_line: 290
end_line: 302
---

# +www.fieldnotes.es 192.0.2.80 300

- Fragment ID: `rgbdns-frag-388087cb9761`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 290-302
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-388087cb9761", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 290, "endLine": 302}
```

## Excerpt

<span id="rgbdns-frag-388087cb9761" class="rgbdns-fragment-target"></span>
### rgbdns-frag-388087cb9761: heading +www.fieldnotes.es 192.0.2.80 300

````markdown
# +www.fieldnotes.es:192.0.2.80:300
```

The address-bearing `cron.sh` NS lines publish the nameserver address records.
The `fieldnotes.es` NS lines intentionally have empty address fields because
their names belong to the separate `cron.sh` zone.

Copy the initial source to the primary login account:

```sh
scp rgbdns.data bitnami@52.10.53.234:rgbdns.data
```

````
