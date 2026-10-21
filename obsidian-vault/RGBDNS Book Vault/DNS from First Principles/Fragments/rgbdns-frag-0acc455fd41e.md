---
type: "code-fragment"
fragment_id: "rgbdns-frag-0acc455fd41e"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "A path through the code"
kind: "heading"
start_line: 2344
end_line: 2365
---

# A path through the code

- Fragment ID: `rgbdns-frag-0acc455fd41e`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 2344-2365
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-0acc455fd41e", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 2344, "endLine": 2365}
```

## Excerpt

<span id="rgbdns-frag-0acc455fd41e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0acc455fd41e: heading A path through the code

```markdown
## A path through the code

Read the project in dependency order:

1. `src/name.rs` — the foundational name invariant.
2. `src/packet.rs` — types and bounded wire codec.
3. `src/zone.rs` — tinydns source and authoritative lookup semantics.
4. `src/cdb.rs` — compiled compatibility format.
5. `src/server.rs` — query validation, answer construction, transport limits.
6. `src/client.rs` — stub behavior and TCP fallback.
7. `src/axfr.rs` — streaming zone movement and atomic installation.
8. `src/dnscache_config.rs` and `src/bin/dnscache.rs` — iterative recursion,
   DNSSEC, forwarding, access, and resource policy.
9. `src/rbl.rs`, `src/pick.rs`, `src/wall.rs`, and `src/special.rs` —
   specialized responders.
10. `src/conf.rs`, `src/setuidgid.rs`, `src/multilog.rs`, and `src/tai64.rs` —
    deployment and operations.

The binaries in `src/bin` should then look thin. That is intentional. They
parse the command contract, load configuration, call a library boundary, print
diagnostics, and map fatal errors to the suite’s exit convention.

```
