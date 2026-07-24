---
type: "code-fragment"
fragment_id: "rgbdns-frag-29f0b4cd6305"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "From query bytes to an authoritative answer"
kind: "heading"
start_line: 1362
end_line: 1390
---

# From query bytes to an authoritative answer

- Fragment ID: `rgbdns-frag-29f0b4cd6305`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1362-1390
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-29f0b4cd6305", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-29f0b4cd6305: heading From query bytes to an authoritative answer", "sourcePath": "docs/book/rgbdns.md", "startLine": 1362, "endLine": 1390}
```

## Excerpt

<span id="rgbdns-frag-29f0b4cd6305" class="rgbdns-fragment-target"></span>
### rgbdns-frag-29f0b4cd6305: heading From query bytes to an authoritative answer

```markdown
# From query bytes to an authoritative answer

[`server::respond`](../../src/server.rs) is the central authoritative pipeline.
Its shape is intentionally linear:

1. Reject an unknown opcode from the header without misparsing its body as a
   standard query.
2. Decode the packet, mapping malformed standard queries to `FORMERR`.
3. Enforce one question and valid OPT placement.
4. Derive the UDP response limit from EDNS and the transport ceiling.
5. Ask `Zone` for a typed `Lookup`.
6. Expand bounded CNAME chains and add relevant target addresses.
7. Normalize RRset TTLs and remove duplicates.
8. Encode or truncate the response.

The code separates mechanism from policy. [`transport.rs`](../../src/transport.rs)
knows UDP datagrams, TCP length prefixes, timeouts, persistent connections, and
a fixed worker bound. It knows nothing about zones. The handler knows DNS
policy but receives transport limits and client identity as ordinary
parameters. That separation lets specialized services reuse the network
machinery without pretending to be authoritative zones.

The original djbdns family achieved robustness partly through small processes.
rgbdns retains that decomposition while strengthening in-process boundaries.
The binaries under [`src/bin`](../../src/bin) are mostly adapters: environment,
configuration, a library call, and the djbdns-compatible fatal exit convention.
Small executables remain independently supervisable, but common logic is
testable as ordinary Rust functions.

```
