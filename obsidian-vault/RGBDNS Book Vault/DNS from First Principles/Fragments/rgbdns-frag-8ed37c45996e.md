---
type: "code-fragment"
fragment_id: "rgbdns-frag-8ed37c45996e"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "ANAME and ACME boundaries"
kind: "heading"
start_line: 882
end_line: 914
---

# ANAME and ACME boundaries

- Fragment ID: `rgbdns-frag-8ed37c45996e`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 882-914
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-8ed37c45996e", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 882, "endLine": 914}
```

## Excerpt

<span id="rgbdns-frag-8ed37c45996e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8ed37c45996e: heading ANAME and ACME boundaries

````markdown
## ANAME and ACME boundaries

A signed answer cannot be synthesized after signing. The publication graph is
therefore visible:

```text
source -> ACME overlay -> selected ANAME materialization
       -> rgbsec-data -> rgbsec-check -> atomic activation
```

For an ANAME in a `K` zone, `aname-materialize` resolves the target before
signing and writes ordinary A and AAAA RRsets under the configured TTL ceiling.
The secondary receives those addresses and signatures; it does not resolve the
target independently for that signed snapshot. ANAME directives in `U` zones
retain the ordinary runtime behavior.

ACME is a privilege-boundary question, not only a record-format question. An
ACME-managed `U` zone can keep using the unprivileged live overlay even while
another zone in the same CDB is signed. Inline updates to a `K` zone would need
an explicitly designed privileged publisher that acknowledges an update only
after a new signed CDB is durable. `rgbdns-setup` does not invent that
escalation. Without such a hook, a signed ACME policy fails closed.

The simplest arrangement is often a small unsigned validation child:

```text
_acme-challenge.example.com.  NS  ns1.validation.example.
```

The production lesson is conservative: choose the first DNSSEC pilot from a
stable zone with ordinary A or AAAA data, no live ACME overlay, and no ANAME.
Move dynamic zones only after their signed publication boundary is explicit.

````
