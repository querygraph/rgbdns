---
type: "code-fragment"
fragment_id: "rgbdns-frag-30baa25c0890"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "13. Publish later primary changes"
kind: "heading"
start_line: 545
end_line: 575
---

# 13. Publish later primary changes

- Fragment ID: `rgbdns-frag-30baa25c0890`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 545-575
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-30baa25c0890", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 545, "endLine": 575}
```

## Excerpt

<span id="rgbdns-frag-30baa25c0890" class="rgbdns-fragment-target"></span>
### rgbdns-frag-30baa25c0890: heading 13. Publish later primary changes

````markdown
## 13. Publish later primary changes

Edit the complete `rgbdns.data` on the workstation and increment the affected
SOA serial. Upload through a temporary name, then atomically rename it:

```sh
scp rgbdns.data bitnami@52.10.53.234:rgbdns.data.new &&
ssh bitnami@52.10.53.234 'mv rgbdns.data.new rgbdns.data'
```

The path watcher:

1. verifies that the final path is a regular file, not a symlink;
2. verifies ownership;
3. copies it into a private staging directory;
4. compiles it as the `rgbdns` service user;
5. replaces live `data` and `data.cdb` only after successful compilation; and
6. restarts tinydns.

Inspect the publication:

```sh
ssh bitnami@52.10.53.234 \
  'sudo journalctl -u rgbdns-data-import.service \
     -u rgbdns-tinydns.service -n 100 --no-pager'
```

A malformed or partially uploaded file does not replace the active database.
The `.new` plus rename sequence prevents the watcher from observing an upload
while `scp` is still writing it.

````
