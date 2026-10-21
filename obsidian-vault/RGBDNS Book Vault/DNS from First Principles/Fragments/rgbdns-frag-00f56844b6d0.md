---
type: "code-fragment"
fragment_id: "rgbdns-frag-00f56844b6d0"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "12. Configure BuddyNS"
kind: "heading"
start_line: 531
end_line: 544
---

# 12. Configure BuddyNS

- Fragment ID: `rgbdns-frag-00f56844b6d0`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 531-544
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-00f56844b6d0", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 531, "endLine": 544}
```

## Excerpt

<span id="rgbdns-frag-00f56844b6d0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-00f56844b6d0: heading 12. Configure BuddyNS

```markdown
## 12. Configure BuddyNS

In BuddyBoard:

1. Add `cron.sh` and `fieldnotes.es`.
2. Set the public primary/master to `52.10.53.234`, port 53.
3. Run the BuddyNS transfer test for both zones.
4. Require the transferred serials to match the primary.
5. Optionally configure `52.38.177.160` as an alternate master only after
   AXFR from `b` is verified.

The application allow-list permits BuddyNS to AXFR from either rgbdns server.
Do not use the VPC address for BuddyNS; it is outside the VPC.

```
