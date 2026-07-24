---
type: "code-fragment"
fragment_id: "rgbdns-frag-d95a4349f84b"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Design patterns to carry elsewhere"
kind: "heading"
start_line: 1164
end_line: 1189
---

# Design patterns to carry elsewhere

- Fragment ID: `rgbdns-frag-d95a4349f84b`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1164-1189
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-d95a4349f84b", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-d95a4349f84b: heading Design patterns to carry elsewhere", "sourcePath": "docs/book/rgbdns.md", "startLine": 1164, "endLine": 1189}
```

## Excerpt

<span id="rgbdns-frag-d95a4349f84b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d95a4349f84b: heading Design patterns to carry elsewhere

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
