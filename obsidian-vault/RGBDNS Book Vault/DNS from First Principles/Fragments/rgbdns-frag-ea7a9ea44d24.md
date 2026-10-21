---
type: "code-fragment"
fragment_id: "rgbdns-frag-ea7a9ea44d24"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Constraints are part of the design"
kind: "heading"
start_line: 985
end_line: 997
---

# Constraints are part of the design

- Fragment ID: `rgbdns-frag-ea7a9ea44d24`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 985-997
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-ea7a9ea44d24", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 985, "endLine": 997}
```

## Excerpt

<span id="rgbdns-frag-ea7a9ea44d24" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ea7a9ea44d24: heading Constraints are part of the design

```markdown
## Constraints are part of the design

rgbdns uses NSEC rather than NSEC3 for authoritative signing. NSEC is smaller,
simpler, and avoids iteration and opt-out machinery; it also makes zone names
enumerable. Location-dependent and time-qualified data cannot be signed because
one owner and type must identify one stable RRset. Pre-existing DNSKEY, RRSIG,
NSEC, and NSEC3 input is rejected. ANAME must be materialized before signing.

These are not footnotes hidden behind a “DNSSEC enabled” switch. They are the
conditions under which an offline immutable snapshot can honestly claim to be
the zone. The small-tool approach makes each condition visible, testable, and
replaceable without putting private keys in the packet-serving process.

```
