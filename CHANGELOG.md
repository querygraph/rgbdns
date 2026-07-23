# Changelog

All notable changes to rgbdns are recorded here. The project follows semantic
versioning once its djbdns-compatible public surface stabilizes.

## [Unreleased]

### Added

- Memory-safe DNS name and packet codecs with bounded compression-pointer,
  section-count, label, RDATA, and TXT parsing.
- Authoritative UDP and TCP server with truncation, wildcard synthesis,
  NXDOMAIN, NODATA, and out-of-zone refusal behavior.
- tinydns data support for `.`, `&`, `=`, `+`, `@`, `'`, `^`, `C`, `Z`, `:`,
  `3`, `6`, and patched `S` records.
- IPv4, IPv6, NS, CNAME, SOA, PTR, MX, TXT, and opaque record support.
- Initial `tinydns`, `tinydns-data`, `tinydns-get`, and `dnsq` commands.
- Compatibility and patch-research ledger.
- Verified public remote at `querygraph/rgbdns`.

### Security

- No unsafe Rust.
- DNS query IDs use operating-system randomness.
- Malformed and cyclic compressed names are rejected with bounded work.
