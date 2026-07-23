# Compatibility and research ledger

Primary baseline: djbdns 1.05, released 2001-02-11 into the public domain.

Implemented:

- RFC 1034/1035 names, compression decoding, queries and resource records
- tinydns markers `. & = + @ ' ^ C Z : 3 6` (location `%` is pending)
- authoritative A, AAAA, NS, CNAME, SOA, PTR, MX, TXT and opaque RR data
- wildcard synthesis, NXDOMAIN/NODATA SOA authority, REFUSED outside zones
- UDP truncation and DNS over TCP
- pointer-loop, section-count, label/name, RDLENGTH, and TXT bounds checks

Patch families being tracked:

- Debian djbdns patch series and `rts.tests`
- Felix von Leitner's IPv6/test patch series
- CVE-2008-4392 merged-query and SOA-cache corrections
- EDNS(0), larger packets, DNSCurve, and tinydnssec/DNSSEC extensions
- errno, libc, compiler, large-file, and platform portability fixes

Remaining suite components:

- recursive `dnscache` with bounded cache, bailiwick enforcement, DNSSEC,
  randomized ports/IDs/case, TCP fallback and query coalescing
- `axfrdns`/`axfr-get`, `rbldns`, `walldns`, and configuration programs
- remaining clients (`dnsip`, `dnsipq`, `dnsname`, `dnsmx`, `dnstxt`,
  `dnsqr`, `dnstrace`, `dnsfilter`)
- byte-for-byte differential tests against patched C djbdns and RFC vectors
