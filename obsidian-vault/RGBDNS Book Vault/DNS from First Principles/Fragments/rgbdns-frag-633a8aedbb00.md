---
type: "code-fragment"
fragment_id: "rgbdns-frag-633a8aedbb00"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "DNSSEC in the tinydns posture"
kind: "heading"
start_line: 712
end_line: 734
---

# DNSSEC in the tinydns posture

- Fragment ID: `rgbdns-frag-633a8aedbb00`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 712-734
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-633a8aedbb00", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 712, "endLine": 734}
```

## Excerpt

<span id="rgbdns-frag-633a8aedbb00" class="rgbdns-fragment-target"></span>
### rgbdns-frag-633a8aedbb00: heading DNSSEC in the tinydns posture

```markdown
## DNSSEC in the tinydns posture

The design goal is not a control plane. It is a set of small programs and
visible files:

| Program | One bounded job |
|---|---|
| `rgbsec-keygen` | create one ECDSA P-256 key and print its policy line |
| `rgbsec-sign` | turn tinydns text into inspectable signed tinydns text |
| `rgbsec-data` | sign and compile directly to a CDB |
| `rgbsec-ds` | derive the parent DS record from one policy line |
| `rgbsec-check` | verify RRsets, signatures, NSEC cycles, and remaining lifetime |

There is no signing daemon, key database, online RPC service, or implicit key
generation in the authority. Each transformation writes beside its destination
and renames only after success. A failed materialization, signature, compile,
reload, or verification leaves the last-known-good CDB active.

The feature is optional. Without a DNSSEC policy and
`/etc/rgbdns/dnssec.env`, `tinydns-data`, `tinydns`, ACME, ANAME, AXFR, and
secondary synchronization retain the djbdns-compatible path. Enabling signing
for one zone does not force every zone in the same CDB to become signed.

```
