---
type: "code-fragment"
fragment_id: "rgbdns-frag-a31a615a1d14"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Design patterns to carry elsewhere"
kind: "heading"
start_line: 1434
end_line: 1459
---

# Design patterns to carry elsewhere

- Fragment ID: `rgbdns-frag-a31a615a1d14`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1434-1459
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-a31a615a1d14", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-a31a615a1d14: heading Design patterns to carry elsewhere", "sourcePath": "docs/book/rgbdns.md", "startLine": 1434, "endLine": 1459}
```

## Excerpt

<span id="rgbdns-frag-a31a615a1d14" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a31a615a1d14: heading Design patterns to carry elsewhere

```markdown
## Design patterns to carry elsewhere

Several rgbdns choices generalize beyond DNS.

**Parse into valid types.** If an invalid name can circulate as an ordinary
string, every consumer must rediscover validation.

**Bound dimensions independently.** A packet byte limit does not replace a
compression-depth limit; a cache byte limit does not replace a recursion-depth
limit.

**Separate policy from mechanism.** `transport.rs` owns bounded UDP and TCP
mechanics while the authoritative and specialized handlers own answer policy.

**Compile mutable source into immutable serving data.** This gives validation,
atomic rollout, simple readers, and easy rollback.

**Preserve protocol distinctions internally.** A `Lookup` enum prevents
NXDOMAIN, NODATA, referral, and refusal from collapsing into “no records.”

**Run in the foreground.** It composes with old and new supervisors and keeps
signals understandable.

**Treat compatibility files as hostile.** Historical layout fidelity need not
mean historical trust assumptions.

```
