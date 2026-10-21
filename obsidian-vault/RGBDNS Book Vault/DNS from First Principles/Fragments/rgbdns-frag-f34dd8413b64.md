---
type: "code-fragment"
fragment_id: "rgbdns-frag-f34dd8413b64"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "15. Verify the public deployment"
kind: "heading"
start_line: 610
end_line: 652
---

# 15. Verify the public deployment

- Fragment ID: `rgbdns-frag-f34dd8413b64`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 610-652
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-f34dd8413b64", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 610, "endLine": 652}
```

## Excerpt

<span id="rgbdns-frag-f34dd8413b64" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f34dd8413b64: heading 15. Verify the public deployment

````markdown
## 15. Verify the public deployment

Compare all rgbdns authorities:

```sh
for server in 52.10.53.234 52.38.177.160; do
  for zone in cron.sh fieldnotes.es; do
    dig @"$server" "$zone" SOA +norecurse
    dig @"$server" "$zone" NS +norecurse
    dig +tcp @"$server" "$zone" SOA +norecurse
  done
done
```

Check each BuddyNS authority:

```sh
for ns in \
  uz5x6wcwzfbjs8fkmkuchydn9339lf7xbxdmnp038cmyjlgg9sprr2.free.ns.buddyns.com \
  uz5dkwpjfvfwb9rh1qj93mtup0gw65s6j7vqqumch0r9gzlu8qxx39.free.ns.buddyns.com \
  uz56xw8h7fw656bpfv84pctjbl9rbzbqrw4rpzdhtvzyltpjdmx0zq.free.ns.buddyns.com
do
  dig @"$ns" fieldnotes.es SOA +norecurse
done
```

Check delegation and glue:

```sh
dig fieldnotes.es NS +trace
dig cron.sh NS +trace
dig a.ns.cron.sh A
dig b.ns.cron.sh A
```

Require:

- `status: NOERROR`;
- the `aa` flag on direct authoritative queries;
- identical current SOA serials;
- matching NS RRsets; and
- UDP and TCP agreement.

````
