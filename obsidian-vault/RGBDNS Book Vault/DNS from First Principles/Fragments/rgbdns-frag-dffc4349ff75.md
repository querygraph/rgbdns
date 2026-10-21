---
type: "code-fragment"
fragment_id: "rgbdns-frag-dffc4349ff75"
source_path: "CHANGELOG.md"
code_note: "DNS from First Principles/Code/CHANGELOG.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "0.6.0 - 2026-08-18"
kind: "heading"
start_line: 64
end_line: 144
---

# 0.6.0 - 2026-08-18

- Fragment ID: `rgbdns-frag-dffc4349ff75`
- Source file: [[DNS from First Principles/Code/CHANGELOG.md.source|CHANGELOG.md]]
- Lines: 64-144
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-dffc4349ff75", "codeNote": "DNS from First Principles/Code/CHANGELOG.md.source", "sourcePath": "CHANGELOG.md", "startLine": 64, "endLine": 144}
```

## Excerpt

<span id="rgbdns-frag-dffc4349ff75" class="rgbdns-fragment-target"></span>
### rgbdns-frag-dffc4349ff75: heading 0.6.0 - 2026-08-18

```markdown
## [0.6.0] - 2026-08-18

### Added

- Opt-in, offline authoritative DNSSEC with ECDSA P-256/SHA-256, NSEC denial,
  signed wildcard and delegation behavior, standard signed AXFR, DS derivation,
  cryptographic snapshot checks, and signed-RRset-aware UDP truncation.
- Tinydns-style `dnssec-keygen`, `dnssec-sign`, `dnssec-data`, `dnssec-ds`,
  `dnssec-check`, `aname-materialize`, and `acme-materialize` utilities with
  line-oriented configuration and atomic last-known-good publication.
- Fail-closed mixed snapshots: `K` lines select signed zones and explicit `U`
  lines retain unsigned zones, including their ANAME and qualified data.
- Root-only signing and unprivileged monitoring timers. DNSSEC remains disabled
  unless an operator supplies a policy; the original rgbdns path is unchanged.

### Security

- Private signing keys are primary-only 0600 files and are never opened by
  `tinydns`, `axfrdns`, or a secondary.
- DS, DNSKEY, RRSIG, and NSEC data are structurally validated at text, CDB,
  packet, and AXFR boundaries before publication or service.

### Fixed

- Daily email delivery no longer requires the optional `hostname` utility when
  a sender is configured, and has portable fallbacks when it is not.

### Added

- A committed FirstPair vault contract now builds non-replacing, standardized
  candidates through the existing RGBDNS builder and strict validator.

- Daily per-zone DNS query reports with total and distinct client/resolver IP
  counts, longest-zone matching, descending totals, journald collection,
  sendmail-compatible delivery, and a hardened systemd timer.

- Scoped TCP RFC 2136 updates authenticated by HMAC-SHA256 TSIG for ACME
  DNS-01 TXT values, with per-key zone/owner policy, replay and resource
  bounds, signed responses, durable overlays, monotonic SOA serials, live
  publication, standard AXFR propagation, canonical-import merge validation,
  Certbot interoperability guidance, and the `rgbdns-acme` administrative
  client.

- Original tinydns-compatible per-request logging for UDP, TCP, malformed,
  refused, class, opcode, and AXFR requests, emitted as untimestamped stderr
  lines for journald or `multilog`, with a default-on `QUERY_LOG` control.
- Multi-zone secondary synchronization from a single primary, with repeatable
  setup arguments, per-zone last-known-good snapshots, and atomic combined CDB
  activation.
- Private ANAME zone directives with CDB persistence, bounded recursive
  CNAME following, TTL-capped address caching, and authoritative apex-safe
  A/AAAA synthesis.
- ANAME miss coalescing and short-lived failure suppression, limiting
  duplicate recursive work and cross-provider loop amplification.
- An RFCXML v3 standards-track ANAME and zone-transfer proposal, with rendered
  text/HTML, provider deployment analysis, native AXFR/IXFR semantics, and a
  reproducible asdf/uv validation toolchain.
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
```
