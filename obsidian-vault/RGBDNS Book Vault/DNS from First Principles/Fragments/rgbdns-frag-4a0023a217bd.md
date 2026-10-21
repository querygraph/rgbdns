---
type: "code-fragment"
fragment_id: "rgbdns-frag-4a0023a217bd"
source_path: "docs/DNSSEC-DESIGN.md"
code_note: "DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Programs"
kind: "heading"
start_line: 14
end_line: 43
---

# Programs

- Fragment ID: `rgbdns-frag-4a0023a217bd`
- Source file: [[DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source|docs/DNSSEC-DESIGN.md]]
- Lines: 14-43
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-4a0023a217bd", "codeNote": "DNS from First Principles/Code/docs/DNSSEC-DESIGN.md.source", "sourcePath": "docs/DNSSEC-DESIGN.md", "startLine": 14, "endLine": 43}
```

## Excerpt

<span id="rgbdns-frag-4a0023a217bd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4a0023a217bd: heading Programs

```markdown
## Programs

`rgbsec-keygen zone keyfile` creates one ECDSA P-256/SHA-256 key in PKCS#8 DER
form. It creates the key with mode 0600 and refuses to overwrite a path. The
only stdout output is the public `K` policy line described below.

`rgbsec-sign [data [data.signed]]` reads tinydns data plus `dnssec` in the
current directory. It resolves no names and changes no live file. It writes a
complete signed tinydns snapshot to a sibling temporary file, syncs it, and
renames it over the requested output. It exits nonzero without changing the
last good output if any input, key, zone, or signature is invalid.

`rgbsec-data [data [data.cdb]]` is the composition used for publication. It
runs the same signing transform and CDB compiler, writes a sibling temporary
CDB, syncs it, validates it by loading it, and renames it over `data.cdb`.
Keeping `rgbsec-sign` separate makes signed text inspectable and lets an
operator compose it with other compilers.

`rgbsec-ds dnssec-line` prints the DS record which must be installed at the
parent. It reads the referenced private key only to derive its public key and
does not modify any file.

`rgbsec-check [data.signed [dnssec]]` performs structural and cryptographic checks and
prints one tab-separated status line per signed zone. Nagios, cron, systemd,
or a shell script can consume the same output. Exit status alone is sufficient
for supervision.

The utility names intentionally describe transformations rather than a suite
manager. Every stage can be invoked and tested independently.

```
