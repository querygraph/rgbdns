---
type: "code-fragment"
fragment_id: "rgbdns-frag-0c7e91de1c2d"
source_path: "CHANGELOG.md"
code_note: "DNS from First Principles/Code/CHANGELOG.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Unreleased"
kind: "heading"
start_line: 6
end_line: 86
---

# Unreleased

- Fragment ID: `rgbdns-frag-0c7e91de1c2d`
- Source file: [[DNS from First Principles/Code/CHANGELOG.md.source|CHANGELOG.md]]
- Lines: 6-86
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-0c7e91de1c2d", "codeNote": "DNS from First Principles/Code/CHANGELOG.md.source", "heading": "rgbdns-frag-0c7e91de1c2d: heading Unreleased", "sourcePath": "CHANGELOG.md", "startLine": 6, "endLine": 86}
```

## Excerpt

<span id="rgbdns-frag-0c7e91de1c2d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0c7e91de1c2d: heading Unreleased

```markdown
## [Unreleased]

### Added

- Private ANAME zone directives with CDB persistence, bounded recursive
  CNAME following, TTL-capped address caching, and authoritative apex-safe
  A/AAAA synthesis.
- Manual pages for every command and packaged service helper, installed by the
  Debian package alongside the rgbdns(7) overview.
- Native Debian packaging with dedicated service accounts, hardened systemd
  units, idempotent primary setup, allow-listed AXFR serving, atomic timed
  secondary synchronization, package build tooling, and an operations guide.
- Cloud Debian package builds with archive inspection, lintian enforcement,
  clean-container installation checks, artifact upload, and automatic
  Cargo-binary discovery.
- Iterative `dnscache` service with DNSSEC validation, root-hint traversal,
  bailiwick enforcement, bounded response/nameserver caches, query
  case-randomization, EDNS, TCP fallback, and loopback-only access by default.
- Self-contained daemontools-compatible `multilog` with streaming input,
  TAI64N line timestamps, symlink-safe append, atomic rotation, bounded
  size/retention controls, and multiple destinations.
- Self-contained `setuidgid` with system account/group resolution,
  supplementary-group initialization, verified UID/GID dropping, and direct
  process replacement.
- Leap-second-aware `tai64n` and `tai64nlocal` stream filters, sharing exact
  TAI64N labels with `multilog` and covering the published daemontools vector.
- Current InterNIC root hints dated 2026-07-22.
- Direct support for original `dnscache` `ROOT/servers/@` files containing
  bounded IPv4/IPv6 server lists, translated through private ephemeral master
  files; BIND-format `ROOTS` files remain supported.
- Original `dnscache` `ROOT/servers/domain` split-horizon forwarding rules,
  with strict filenames, bounded files/address counts, UDP-to-TCP fallback,
  isolated caches, and longest-suffix catalog selection.
- Memory-safe DNS name and packet codecs with bounded compression-pointer,
  section-count, label, RDATA, and TXT parsing.
- Authoritative UDP and TCP server with truncation, wildcard synthesis,
  NXDOMAIN, NODATA, and out-of-zone refusal behavior.
- EDNS(0) payload negotiation, DO-bit echo, BADVERS responses, and validated
  option framing.
- Child-zone referrals with authoritative-bit handling and in-bailiwick glue.
- Whole-record UDP truncation while preserving as many answers as fit; TCP
  responses are not subjected to the legacy 512-byte UDP ceiling.
- tinydns data support for `.`, `&`, `=`, `+`, `@`, `'`, `^`, `C`, `Z`, `:`,
  `3`, `6`, and patched `S` records.
- Felix von Leitner-compatible flat 32-hex-digit IPv6 data fields: `6` emits
  AAAA plus `ip6.arpa` and historical `ip6.int` PTR records, while `3` emits
  AAAA only.
- IPv4, IPv6, NS, CNAME, SOA, PTR, MX, TXT, and opaque record support.
- Typed IPv4/IPv6 listen-address construction across every daemon, including
  correct bracketed socket addresses for `IP=::` and other IPv6 literals.
- Initial `tinydns`, `tinydns-data`, `tinydns-get`, and `dnsq` commands.
- Atomic `tinydns-data` output in the original `data.cdb` key/value format and
  a bounded, validating CDB loader used by `tinydns` by default.
- Query-time TAI64 activation/expiration semantics and longest-prefix `%`
  client-location selection, retained exactly through text and CDB forms.
- Source-file modification times for implicit SOA serials, matching
  `tinydns-data`, plus enforcement of its generic-record type restrictions.
- RFC closest-encloser wildcard behavior and empty-nonterminal NODATA handling,
  with an indexed fast path for ordinary unqualified zones.
- Bounded in-zone CNAME-chain expansion with loop-to-SERVFAIL handling and
  address additionals for NS, MX, and SRV targets.
- Minimal bounded FORMERR replies for decodable malformed queries, NOTIMP for
  unsupported opcodes, and rejection of duplicate EDNS OPT records.
- Fixed-size TCP accept-worker pools and per-connection deadlines for
  authoritative, specialized, and AXFR services; AXFR also enforces message,
  record, and aggregate-byte limits.
- Client-IP selection in `tinydns-get`.
- Correct original djbdns field positions, default target expansion, glue
  generation, SOA TTL behavior, one-to-three-digit octal escapes, and patched
  SRV priority/weight ordering, differentially checked against patched 1.05.
- TCP-only `axfrdns` with CIDR access controls, bounded multi-message transfers,
  authoritative-zone isolation, and matching opening/closing SOA records.
- `axfr-get` with strict response validation, transfer limits, safe tinydns
  escaping, fsync, and atomic output replacement.
- `rbldns` and atomic `rbldns-data`, including longest-prefix IPv4 block-list
  matching, configurable A/TXT responses, `$` address substitution, and exact
  CDB key/value compatibility with the original compiler.
- `walldns` direct and `in-addr.arpa` A/PTR mappings with the original TTL and
  refusal behavior.
- Location-aware `pickdns`, atomic `pickdns-data`, and `pickdns-conf`, with
  unbiased address shuffling, original three-answer/TTL behavior, fallback
```
