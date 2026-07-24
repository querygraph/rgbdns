---
type: "code-fragment"
fragment_id: "rgbdns-frag-54eee914935f"
source_path: "docs/blog/announcing-rgbdns/post.md"
code_note: "DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "A suite of small programs"
kind: "heading"
start_line: 21
end_line: 42
---

# A suite of small programs

- Fragment ID: `rgbdns-frag-54eee914935f`
- Source file: [[DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source|docs/blog/announcing-rgbdns/post.md]]
- Lines: 21-42
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-54eee914935f", "codeNote": "DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source", "heading": "rgbdns-frag-54eee914935f: heading A suite of small programs", "sourcePath": "docs/blog/announcing-rgbdns/post.md", "startLine": 21, "endLine": 42}
```

## Excerpt

<span id="rgbdns-frag-54eee914935f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-54eee914935f: heading A suite of small programs

```markdown
## A suite of small programs

The original djbdns design did not hide every DNS role inside one long-running
binary. It used small foreground programs with explicit jobs. rgbdns preserves
that operational shape while replacing the unsafe implementation substrate
with Rust:

- `tinydns` serves authoritative zones over UDP and TCP.
- `tinydns-data` compiles readable source into an immutable CDB.
- `dnscache` performs bounded recursion with DNSSEC validation.
- `axfrdns` and `axfr-get` transfer zones over framed TCP.
- `rbldns`, `walldns`, and `pickdns` provide specialized answer policies.
- `dnsq`, `dnsip`, `dnsname`, `dnsmx`, `dnstxt`, `dnstrace`, and related tools
  make the protocol inspectable from the command line.
- `multilog`, `setuidgid`, `tai64n`, and `tai64nlocal` provide a
  self-contained foreground-service runtime.

The binaries stay thin. Shared library boundaries own validated names, typed
records, packet encoding, zone semantics, CDB compatibility, transport, and
client response matching. This is not a monolith cut into arbitrary commands;
it is one set of protocol invariants composed into several purposes.

```
