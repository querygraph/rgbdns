# Compatibility and research ledger

Primary baseline: djbdns 1.05, released 2001-02-11 into the public domain.

Implemented:

- RFC 1034/1035 names, compression decoding, queries and resource records
- tinydns markers `. & = + @ ' ^ C Z : 3 6 S` (location `%` is pending)
- atomic djbdns-compatible `data.cdb` compilation and bounded, validating CDB
  loading; a 14-record fixture was differentially verified as identical
  key/value entries against Debian-patched djbdns 1.05
- authoritative A, AAAA, NS, CNAME, SOA, PTR, MX, TXT and opaque RR data
- wildcard synthesis, NXDOMAIN/NODATA SOA authority, REFUSED outside zones
- referrals with in-bailiwick glue and correct authoritative-bit behavior
- EDNS(0), BADVERS, DO-bit echo, whole-record UDP truncation, and DNS over TCP
- pointer-loop, section-count, label/name, RDLENGTH, and TXT bounds checks
- randomized property tests and live UDP/TCP integration tests
- iterative DNSSEC-validating `dnscache`, verified live against signed
  `cloudflare.com` (AD), authenticated denial, and deliberately bogus
  `dnssec-failed.org` (SERVFAIL without AD); the opt-in test is
  `cargo test --test dnscache_network -- --ignored`
- bounded RFC 5936-style AXFR serving and retrieval, with atomic conversion
  back to tinydns text and loopback-only service access by default

Patch/source corpus searched and tracked:

- Upstream djbdns 1.05 source and documentation at <https://cr.yp.to/djbdns.html>.
- Debian 1.05-22.1 source, Python autopkgtests, historical `rts.tests`, and
  patches for long-packet compression, SRV/PTR, recursion depth, the
  CVE-2012-1191 ghost-domain attack, query merging, SOA caching, root hints,
  data limits, POSIX types, and modern compilers.
- Felix von Leitner's IPv6 patch line through test32.
- FreeBSD's jumbo-p13, persistent-cache, ignoreip2, persistent-mmap, SRV,
  DNSCurve, and IPv6 options.
- NetBSD's cache-statistics, ignoreip2, multi-listener, merge-query, and
  tinydns 64-bit patch options.
- Gentoo's merged patch set including CVE-2008-4392 query coalescing and SOA
  caching.
- tinydnssec's DNSSEC records, signing workflow, EDNS(0), and large-response
  support.
- Third-party rbldns per-entry-response and multiple-zone patches.
- errno, libc, compiler, large-file, and platform portability fixes across
  Debian, Gentoo, FreeBSD, NetBSD, Ubuntu, and openSUSE packaging.

Remaining suite components:

- djbdns `dnscache` filesystem configuration compatibility and cache-dump
  tooling (the running iterative engine already provides bounded caches,
  bailiwick enforcement, DNSSEC, randomized ports/IDs/case, TCP fallback,
  query coalescing, and negative caching)
- `rbldns`, `walldns`, and configuration programs
- remaining clients (`dnsip`, `dnsipq`, `dnsname`, `dnsmx`, `dnstxt`,
  `dnsqr`, `dnstrace`, `dnsfilter`)
- automated differential tests against patched C djbdns and additional RFC
  vectors (the initial CDB corpus has been manually verified)
