---
type: "code-fragment"
fragment_id: "rgbdns-frag-ca8e2e56fa9a"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Preface {-}"
kind: "heading"
start_line: 1
end_line: 21
---

# Preface {-}

- Fragment ID: `rgbdns-frag-ca8e2e56fa9a`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1-21
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-ca8e2e56fa9a", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-ca8e2e56fa9a: heading Preface {-}", "sourcePath": "docs/book/rgbdns.md", "startLine": 1, "endLine": 21}
```

## Excerpt

<span id="rgbdns-frag-ca8e2e56fa9a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ca8e2e56fa9a: heading Preface {-}

```markdown
# Preface {-}

DNS is often introduced as “the Internet’s phone book.” That metaphor is
useful for about a minute. A phone book is one database, published in editions,
mapping people to telephone numbers. DNS is a distributed protocol for finding
typed, time-limited statements in a delegated tree. It has many writers, many
readers, caches between them, multiple transports, and rules for proving both
presence and absence. Names can point to addresses, but they can also identify
mail exchangers, service endpoints, authoritative servers, cryptographic keys,
and arbitrary text.

This book develops DNS from those underlying problems. The first half builds a
mental model independent of any implementation. The second half walks through
rgbdns, a memory-safe Rust reimplementation of the djbdns suite. The aim is not
only to explain what each program does, but why its boundaries look the way
they do: immutable compiled data for authority, a separate recursive cache,
small diagnostic clients, foreground daemons, and stream-oriented logging.

The code is the final authority for rgbdns behavior. This book describes
version 0.1.0 as built on 2026-07-23.

```
