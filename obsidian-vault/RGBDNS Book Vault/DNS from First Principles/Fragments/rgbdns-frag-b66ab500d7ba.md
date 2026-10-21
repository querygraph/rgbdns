---
type: "code-fragment"
fragment_id: "rgbdns-frag-b66ab500d7ba"
source_path: "docs/blog/rgbdns-dnssec/post.md"
code_note: "DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "DNSSEC in the spirit of tinydns"
kind: "heading"
start_line: 1
end_line: 23
---

# DNSSEC in the spirit of tinydns

- Fragment ID: `rgbdns-frag-b66ab500d7ba`
- Source file: [[DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source|docs/blog/rgbdns-dnssec/post.md]]
- Lines: 1-23
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-b66ab500d7ba", "codeNote": "DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source", "sourcePath": "docs/blog/rgbdns-dnssec/post.md", "startLine": 1, "endLine": 23}
```

## Excerpt

<span id="rgbdns-frag-b66ab500d7ba" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b66ab500d7ba: heading DNSSEC in the spirit of tinydns

```markdown
# DNSSEC in the spirit of tinydns

*August 2026 — rgbdns 0.6.3*

![Polynesian islands protected by double-hulled canoes and Kon-Tiki-style rafts, linked by electric pulses carried through the sea by eels and through the air by flying fish.](rgbdns-dnssec-islands.png)

DNSSEC can look like an invitation to build a control plane: a key database, a
signing service, an RPC protocol, a stateful rollover controller, and an online
authority that knows too much. That would be one way to add cryptographic
signatures to DNS. It would not be the tinydns way.

rgbdns 0.6.3 now supports authoritative DNSSEC as a composition of small
utilities, one-line policy, immutable snapshots, and atomic publication. The
ordinary server never opens a private key. A secondary never receives one. A
zone without DNSSEC continues through the original djbdns-compatible path.

The image above is the mental model I ended up with. Each island is a zone. The
boats around it are independent authorities carrying the same signed facts.
The electric pulses are signatures and authenticated transfers. Eels keep the
underwater path alive; flying fish bridge gaps in the air. The protection does
not come from one enormous ship. It comes from a visible network whose members
can be checked independently.

```
