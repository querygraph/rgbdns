# Announcing rgbdns: DNS from first principles, rebuilt in Rust

*July 2026 — rgbdns 0.1.1*

DNS is often introduced as the internet's phone book. That metaphor is easy to
remember and almost immediately too small. DNS is a delegated publication
system: a global tree of names, administrative cuts, typed statements,
cacheable lifetimes, referrals, recursive discovery, and authenticated chains.
Its wire format is compact enough to reward careful engineering and hostile
enough to punish casual parsing.

**rgbdns** is a memory-safe Rust reimplementation of the djbdns program family,
and **DNS from First Principles** is its new First Pair Press book. The code and
the book were built together. The book derives DNS from the problem it solves,
then follows those ideas into the actual types, packet codec, authoritative
server, recursive cache, zone compiler, transfer tools, diagnostics, logging,
and service supervision in rgbdns.

![A network of luminous paths joining cities across a dark world, the visual language of DNS delegation and resolution.](../../../cover/rgbdns-headboard.png)

## A suite of small programs

The original djbdns design did not hide every DNS role inside one long-running
binary. It used small foreground programs with explicit jobs. rgbdns preserves
that operational shape while replacing the unsafe implementation substrate
with Rust:

- `tinydns` serves authoritative zones over UDP and TCP.
- `tinydns-data` compiles readable source into an immutable CDB.
- `dnscache` performs bounded recursion with DNSSEC validation.
- `axfrdns` and `axfr-get` transfer zones over framed TCP.
- `rbldns`, `walldns`, and `pickdns` provide specialized answer policies.
- `dnsq`, `dnsip`, `dnsname`, `dnsmx`, `dnstxt`, `dnstrace`, and related tools
  make the protocol inspectable from the command line.
- `multilog`, `setuidgid`, `tai64n`, and `tai64nlocal` provide a
  self-contained foreground-service runtime.

The binaries stay thin. Shared library boundaries own validated names, typed
records, packet encoding, zone semantics, CDB compatibility, transport, and
client response matching. This is not a monolith cut into arbitrary commands;
it is one set of protocol invariants composed into several purposes.

## Compatibility without inherited memory hazards

rgbdns reads the tinydns source language and the original `data.cdb` key/value
layout. It supports the familiar record markers, location-aware answers,
query-time TAI64 activation and expiration, patched IPv6 and SRV forms, and
the original suite's service-directory conventions.

Compatibility is checked rather than assumed. Golden fixtures compare compiled
CDB entries with the patched C implementation. Tests retain the awkward parts:
field positions, default target expansion, escaped colons, octal text, SOA TTL
rules, wildcard behavior, client-location selection, IPv6 reverse trees, and
SRV ordering.

The compatibility boundary is deliberately narrow. Historical files are
accepted as hostile input, parsed into validated Rust types, and served through
bounded code. The implementation preserves the useful contract without
preserving the old trust assumptions.

## Conformance is a test matrix

“RFC compliant” is not a useful finish line unless each claim has an oracle.
rgbdns now carries a traceable conformance matrix covering the implemented
surface of RFC 1035, 2181, 2308, 3597, 4343, 4592, 5936, 6891, 7766, 8482,
8906, and 9619.

The tests distinguish details that disappear in broad success/failure checks:

- NXDOMAIN from NODATA, with the correct negative SOA lifetime;
- an unknown record type from an unknown opcode;
- legal unknown EDNS options from malformed option framing;
- a backward compression pointer from a pointer into arbitrary earlier bytes;
- a wildcard synthesis point from an existing empty non-terminal;
- a referral from an authoritative answer, with only in-bailiwick glue;
- an AXFR stream with matching SOA bookends from a plausible but unrelated
  response;
- a truncated UDP message from a complete persistent TCP response.

The RFC cases are joined by hostile-wire corpora and forty thousand generated
property cases per full run. Arbitrary packets must never panic. Every accepted
packet must reparse stably. Structured messages must survive semantic
round-trips. Changing ASCII letter case must not change DNS name identity.
Every truncated prefix of a valid structured packet must be rejected.

An independent ldns `drill` test launches the real `tinydns` binary and makes
UDP, TCP, EDNS, mixed-case, and unknown-type requests. That boundary matters:
rgbdns cannot accidentally pass by making the same mistake in its own encoder
and decoder.

## Hardening that came from the tests

The conformance work changed the design.

The decoder now records valid prior name boundaries, so compression pointers
must target an actual earlier name occurrence. Stub replies are bound to their
request ID, response bit, opcode, and exact question. AXFR adds authoritative
and truncation checks, question rules, zone confinement, and matching opening
and closing SOAs.

Zone loading rejects a CNAME that coexists with other data or points at
multiple different targets. RRsets are normalized to their minimum TTL and
deduplicated before transmission. EDNS placement, count, version, payload, and
option framing are checked explicitly.

UDP and TCP servers share one bounded transport implementation. TCP uses a
fixed worker pool, per-connection deadlines, persistent framing, and pipelined
queries. The result is less duplicated socket code and a stronger common
service contract.

## Measure before calling it faster

rgbdns includes a dependency-free stable-Rust benchmark for packet decoding,
packet encoding, exact and negative zone lookup, authoritative response
construction, and large-response truncation.

On the July 2026 aarch64 Android checkpoint:

- a 64-record response shrank from 2,147 bytes to 1,059 bytes;
- decoding that response improved from 52,661 ns to 29,540 ns;
- an absent-name lookup in a 1,000-name zone improved from 29,889 ns to
  2,726 ns;
- a small authoritative response improved from 17,007 ns to 7,714 ns;
- truncating a 200-record response improved from 3,098,232 ns to 2,570,077 ns.

The numbers also preserve an unfavorable result. Encoding the compressed
64-record response takes 5,309 ns instead of the uncompressed writer's
2,318 ns. That extra CPU buys roughly half the wire bytes and faster downstream
decoding. It is a reasonable DNS tradeoff, but only if it remains visible.

The largest speedup comes from an index of every zone node, including empty
non-terminals. A clearly absent name no longer scans the records of a
thousand-name zone. Truncation searches how many tail records must be removed
instead of encoding after every single removal. Name compression records
suffixes, while a last-owner cache makes repeated RRset owners cheap.

## Running without `supervise`

rgbdns still runs correctly under daemontools, but it does not require an old
supervision stack.

For an existing Linux host, systemd is the practical default: foreground
processes, explicit users, restart policy, limits, capability controls, and
central logs. runit is the closest migration when service directories and
`run` scripts are part of the operational model. s6 with s6-rc is the strongest
choice when dependency-aware supervision and a deliberately composed service
graph matter. OpenRC fits systems that already use it, while Kubernetes and
similar orchestrators should supervise one foreground responsibility per
container rather than nesting another restart manager inside the pod.

The key contract is portable: stay in the foreground, log to standard streams,
replace the process directly, expose readiness honestly, and let one supervisor
own restarts.

## Read the book, inspect the source

**DNS from First Principles** is published as a complete First Pair Press
edition:

- [Library page](https://firstpair.org/books/rgbdns/)
- [Read online](https://firstpair.org/read/rgbdns/)
- [Chapter reader](https://firstpair.org/read/rgbdns/chapters/)
- [PDF](https://firstpair.org/rgbdns/pdf/)
- [EPUB](https://firstpair.org/rgbdns/epub/)
- [Source](https://github.com/querygraph/rgbdns)

The book begins with identity and delegation rather than commands. It builds
names, zones, records, messages, transport, authority, recursion, caching,
DNSSEC, and AXFR from their constraints. Only then does it walk through the
rgbdns program family, operations, security model, conformance evidence,
benchmarks, and supervisor choices.

DNS looks simple from the application boundary because a great deal of
distributed machinery agrees to make it look simple. rgbdns makes that
machinery small enough to inspect, strict enough to test, and safe enough to
run.
