# Changelog

All notable changes to rgbdns are recorded here. The project follows semantic
versioning once its djbdns-compatible public surface stabilizes.

## [Unreleased]

### Added

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
  locations, and differentially identical CDB entries.
- Shared bounded stub resolver with OS-random query IDs, connected UDP source
  validation, retries, resolver configuration, and automatic TCP fallback.
- Runnable `dnsip`, `dnsipq`, `dnsname`, `dnsmx`, `dnstxt`, and `dnsqr`
  clients; `dnsq` now uses the hardened shared transport.
- Patched-suite `dnsip6` and `dnsip6q` AAAA lookup clients.
- Bounded-concurrency, order-preserving `dnsfilter` and an IPv4/IPv6-capable
  iterative `dnstrace` with referral/glue reporting and depth limits.
- Atomic `tinydns-edit add` operations with original duplicate checks, TTL
  inheritance, permissions, and NS/MX slot allocation, differentially verified
  against djbdns.
- Patched `tinydns-edit` `host6` and `alias6` modes with unambiguous flat IPv6
  output and address-family validation.
- OS-randomized, non-repeating `random-ip` generation with fixed-prefix and
  count bounds.
- Portable `tinydns-conf`, `dnscache-conf`, `rbldns-conf`, `walldns-conf`, and
  `axfrdns-conf` service-directory generators with non-overwrite behavior,
  executable run/log scripts, current root hints, and private random seeds.
- Generated service directories use absolute paths to rgbdns's `multilog` and
  `setuidgid`; external daemontools executables are no longer required.
- Permanent patched-C golden-entry tests for tinydns, rbldns, and pickdns CDB
  output; the tinydns corpus includes 19 exact semantic entries covering
  location, timestamp, SRV, AAAA, `ip6.arpa`, and `ip6.int` behavior.
- Compatibility and patch-research ledger.
- Verified public remote at `querygraph/rgbdns`.
- Least-privilege GitHub Actions verification for formatting, strict clippy,
  every offline test target, and optimized production binaries.
- Clean Rust 1.97 clippy compatibility and the Node 24-based checkout action.

### Security

- No unsafe Rust.
- DNS query IDs use operating-system randomness.
- Malformed and cyclic compressed names are rejected with bounded work.
- Non-backward compression pointers and trailing packet data are rejected.
- Property tests exercise 20,000 arbitrary and round-trip packet cases per
  complete test run.
- `dnscache` rejects malformed and out-of-range cache, recursion, and
  access-list settings instead of silently defaulting or allocating without a
  configured ceiling.
- All shipped command-line paths report malformed input and I/O failures
  without Rust panics; this includes hardened `dnsq` and `tinydns-get` paths.
- Portable explicit error conversion for `random-ip` OS-randomness failures,
  including targets where `getrandom::Error` does not implement the standard
  error trait.
- Resolver dependencies pinned to Hickory 0.26.1, which contains the fixes for
  RUSTSEC-2026-0119 (encoder CPU exhaustion) and RUSTSEC-2026-0120 (unbounded
  NSEC3 closest-encloser validation), and uses the response-keyed recursor
  architecture that supersedes RUSTSEC-2026-0106.
