---
type: "code-fragment"
fragment_id: "rgbdns-frag-671f457bf51b"
source_path: "docs/DNSSEC-DESIGN.md"
code_note: "DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Composition with ANAME and ACME"
kind: "heading"
start_line: 122
end_line: 150
---

# Composition with ANAME and ACME

- Fragment ID: `rgbdns-frag-671f457bf51b`
- Source file: [[DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source|docs/DNSSEC-DESIGN.md]]
- Lines: 122-150
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-671f457bf51b", "codeNote": "DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source", "sourcePath": "docs/DNSSEC-DESIGN.md", "startLine": 122, "endLine": 150}
```

## Excerpt

<span id="rgbdns-frag-671f457bf51b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-671f457bf51b: heading Composition with ANAME and ACME

````markdown
## Composition with ANAME and ACME

A signed snapshot cannot contain answers synthesized after signing. The
publication pipeline is therefore explicit:

```text
source -> acme overlay -> selected ANAME materialize -> rgbsec-data -> verify -> rename
```

Each arrow is a program reading a stable input and producing a new sibling
file. Failure leaves the prior CDB live. The signer performs no recursive DNS
lookups.

For ACME, delegation of `_acme-challenge` to a small unsigned validation zone is
the simplest deployment. An ACME policy for a `U` zone remains an unprivileged
live overlay even if another zone in the snapshot is signed. When an inline
signed validation zone is required, the update receiver writes its overlay
atomically and invokes an explicitly configured privileged publication command
synchronously; an update is acknowledged only after the new signed CDB is
durable. Setup does not synthesize that privilege boundary. The command is
replaceable and receives paths, not private key bytes.

For ANAME, a separate materializer resolves targets only inside `K` zones,
enforces the configured TTL ceiling, writes ordinary address records, and
records their earliest expiry. ANAME directives in `U` zones remain ordinary
runtime rgbdns ANAME data. The signing/publishing schedule must refresh before
that expiry. Secondaries receive the materialized records and their signatures;
they do not resolve the target independently for a signed zone.

````
