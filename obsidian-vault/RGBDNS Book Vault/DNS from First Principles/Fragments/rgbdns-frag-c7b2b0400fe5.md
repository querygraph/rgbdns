---
type: "code-fragment"
fragment_id: "rgbdns-frag-c7b2b0400fe5"
source_path: "docs/blog/operationalizing-rgbdns/post.md"
code_note: "DNS from First Principles/Code/docs/blog/operationalizing-rgbdns/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "rgbdns is operational Debian, openSUSE, and ANAME across AXFR"
kind: "heading"
start_line: 1
end_line: 25
---

# rgbdns is operational Debian, openSUSE, and ANAME across AXFR

- Fragment ID: `rgbdns-frag-c7b2b0400fe5`
- Source file: [[DNS from First Principles/Code/docs/blog/operationalizing-rgbdns/post.md.source|docs/blog/operationalizing-rgbdns/post.md]]
- Lines: 1-25
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-c7b2b0400fe5", "codeNote": "DNS from First Principles/Code/docs/blog/operationalizing-rgbdns/post.md.source", "sourcePath": "docs/blog/operationalizing-rgbdns/post.md", "startLine": 1, "endLine": 25}
```

## Excerpt

<span id="rgbdns-frag-c7b2b0400fe5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c7b2b0400fe5: heading rgbdns is operational Debian, openSUSE, and ANAME across AXFR

```markdown
# rgbdns is operational: Debian, openSUSE, and ANAME across AXFR

*July 2026 — rgbdns 0.3.3*

![An openSUSE chameleon launches an RPM package spacecraft toward the Moon while a red Debian spiral forms a galaxy in deep space.](operationalizing-rgbdns-headboard.png)

The first rgbdns release established the larger idea: rebuild the djbdns
program family in memory-safe Rust without losing the small-program design,
the tinydns data format, or the ability to understand the entire authoritative
serving path. The newest release turns that implementation into something an
operator can deploy and keep running.

rgbdns 0.3.3 ships installable Debian and openSUSE packages, systemd services
for primary and secondary authority, atomic file-drop workflows for zone data,
multi-zone AXFR synchronization, and query logging compatible with both the
system journal and `multilog`. It also carries a working implementation of
ANAME address flattening that preserves the ANAME source record between
upgraded rgbdns authorities during AXFR.

That last part is now also a standards proposal. The complete design has been
submitted as the individual Internet-Draft
[Address-specific DNS Aliases (ANAME) and Zone Transfer](https://datatracker.ietf.org/doc/draft-khrabrov-dnsop-aname-axfr/).
It is an active proposal for discussion, not an approved RFC or an IETF
endorsement.

```
