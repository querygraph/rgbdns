---
type: "code-fragment"
fragment_id: "rgbdns-frag-9ba03b400fbd"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Lifecycle and safe rollback"
kind: "heading"
start_line: 958
end_line: 984
---

# Lifecycle and safe rollback

- Fragment ID: `rgbdns-frag-9ba03b400fbd`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 958-984
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-9ba03b400fbd", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 958, "endLine": 984}
```

## Excerpt

<span id="rgbdns-frag-9ba03b400fbd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9ba03b400fbd: heading Lifecycle and safe rollback

```markdown
## Lifecycle and safe rollback

DNSSEC adds time to the authority contract. Monitor the publisher and checker,
the earliest RRSIG expiration, key tag, parent DS, primary/secondary serials,
and both positive and negative validation. Restart each authority during the
pilot and prove that the last signed CDB remains available. Before the first
signature window closes, observe an automatic refresh and verify the new
inception and expiration on both authorities.

Version 0.6.3 deliberately supports one active combined signing key per zone.
It does not pretend that replacing a key file is a rollover protocol. A future
multi-key release must represent publication, DS overlap, retirement, and
recovery as explicit states.

Rollback order is asymmetric. Before DS activation, the operator may return to
the unsigned path because no validator expects signatures. After DS activation:

1. remove the DS at the parent;
2. wait through the parent DS TTL, negative caches, and relevant resolver
   caches;
3. confirm that independent resolvers no longer observe the DS; and only then
4. disable signing and publish unsigned data.

Removing signatures while the DS remains visible converts the zone from secure
to bogus. Keep the retired private key backed up until the DS removal and cache
window are conclusively complete.

```
