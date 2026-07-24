---
type: "code-fragment"
fragment_id: "rgbdns-frag-4c85445184dc"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Identity is not location"
kind: "heading"
start_line: 24
end_line: 53
---

# Identity is not location

- Fragment ID: `rgbdns-frag-4c85445184dc`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 24-53
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-4c85445184dc", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-4c85445184dc: heading Identity is not location", "sourcePath": "docs/book/rgbdns.md", "startLine": 24, "endLine": 53}
```

## Excerpt

<span id="rgbdns-frag-4c85445184dc" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4c85445184dc: heading Identity is not location

```markdown
## Identity is not location

A network delivers packets to addresses. Humans and applications want stable
identities. Those two things should not be fused.

Suppose a service is reached at `192.0.2.8`. If that address is embedded in
every configuration, moving the service requires changing every client. A name
such as `api.example` introduces indirection:

```text
application → api.example → 192.0.2.8 → packets
```

Indirection has a cost: another system must answer the middle question. Its
benefit is that the service owner can change the answer without changing the
application. DNS is the globally deployed mechanism for this indirection.

The mapping is not a function from one name to one address. One name may have
several addresses. The answers may differ by client location. A mail domain
may name several mail exchangers with preferences. A service may delegate a
subtree to another organization. The useful abstraction is therefore:

```text
(owner name, record type, class) → a set of resource records
```

The owner and type together select an RRset. “RRset” means all resource records
with the same owner, type, and class. Implementations should normally treat the
set as a unit because caches and DNSSEC signatures do.

```
