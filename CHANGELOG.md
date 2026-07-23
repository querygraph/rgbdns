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
- Shared bounded stub resolver with OS-random query IDs, connected UDP source
  validation, retries, resolver configuration, and automatic TCP fallback.
- Runnable `dnsip`, `dnsipq`, `dnsname`, `dnsmx`, `dnstxt`, and `dnsqr`
  clients; `dnsq` now uses the hardened shared transport.
- Bounded-concurrency, order-preserving `dnsfilter` and an IPv4/IPv6-capable
  iterative `dnstrace` with referral/glue reporting and depth limits.
- Compatibility and patch-research ledger.
- Verified public remote at `querygraph/rgbdns`.

### Security

- No unsafe Rust.
- DNS query IDs use operating-system randomness.
- Malformed and cyclic compressed names are rejected with bounded work.
- Property tests exercise 20,000 arbitrary and round-trip packet cases per
  complete test run.
