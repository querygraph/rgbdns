# Changelog

All notable changes to rgbdns are recorded here. The project follows semantic
versioning once its djbdns-compatible public surface stabilizes.

## [Unreleased]

### Added

- Iterative `dnscache` service with DNSSEC validation, root-hint traversal,
  bailiwick enforcement, bounded response/nameserver caches, query
  case-randomization, EDNS, TCP fallback, and loopback-only access by default.
- Current InterNIC root hints dated 2026-07-22.
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
- IPv4, IPv6, NS, CNAME, SOA, PTR, MX, TXT, and opaque record support.
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
  locations, and differentially identical CDB entries.
- Shared bounded stub resolver with OS-random query IDs, connected UDP source
  validation, retries, resolver configuration, and automatic TCP fallback.
- Runnable `dnsip`, `dnsipq`, `dnsname`, `dnsmx`, `dnstxt`, and `dnsqr`
  clients; `dnsq` now uses the hardened shared transport.
- Bounded-concurrency, order-preserving `dnsfilter` and an IPv4/IPv6-capable
  iterative `dnstrace` with referral/glue reporting and depth limits.
- Atomic `tinydns-edit add` operations with original duplicate checks, TTL
  inheritance, permissions, and NS/MX slot allocation, differentially verified
  against djbdns.
- OS-randomized, non-repeating `random-ip` generation with fixed-prefix and
  count bounds.
- Portable `tinydns-conf`, `dnscache-conf`, `rbldns-conf`, `walldns-conf`, and
  `axfrdns-conf` service-directory generators with non-overwrite behavior,
  executable run/log scripts, current root hints, and private random seeds.
- Compatibility and patch-research ledger.
- Verified public remote at `querygraph/rgbdns`.

### Security

- No unsafe Rust.
- DNS query IDs use operating-system randomness.
- Malformed and cyclic compressed names are rejected with bounded work.
- Non-backward compression pointers and trailing packet data are rejected.
- Property tests exercise 20,000 arbitrary and round-trip packet cases per
  complete test run.
