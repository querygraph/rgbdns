---
type: "code-fragment"
fragment_id: "rgbdns-frag-0d0e813e38f5"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Errors are protocol results"
kind: "heading"
start_line: 300
end_line: 320
---

# Errors are protocol results

- Fragment ID: `rgbdns-frag-0d0e813e38f5`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 300-320
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-0d0e813e38f5", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-0d0e813e38f5: heading Errors are protocol results", "sourcePath": "docs/book/rgbdns.md", "startLine": 300, "endLine": 320}
```

## Excerpt

<span id="rgbdns-frag-0d0e813e38f5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0d0e813e38f5: heading Errors are protocol results

```markdown
## Errors are protocol results

Several results that sound similar are materially different:

- **NOERROR with answers**: the requested RRset exists.
- **NOERROR without answers**, often called NODATA: the name exists but the
  requested type does not.
- **NXDOMAIN**: the queried name does not exist.
- **SERVFAIL**: the server could not safely complete processing.
- **REFUSED**: policy forbids the operation.
- **FORMERR**: the message is malformed.
- **NOTIMP**: the requested opcode is unsupported.

Negative answers normally include the zone’s SOA so resolvers can cache the
negative result. Confusing NODATA with NXDOMAIN can suppress other valid types
at the same name.

rgbdns expresses authoritative lookup outcomes as `Lookup::Answer`,
`Referral`, `NoData`, `NxDomain`, and `Refused`. That internal sum type forces
the response builder to handle each protocol meaning explicitly.

```
