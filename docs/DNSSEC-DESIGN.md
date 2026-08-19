# DNSSEC design: small tools and visible state

Status: implementation contract for rgbdns 0.6.0.

rgbdns DNSSEC follows the tinydns posture: each program does one bounded job,
configuration is one record per line, persistent state is an ordinary file, and
publication is an atomic rename. There is no signing daemon, key database, RPC
control plane, or implicit key generation in the authoritative server.

The cryptographic implementation may use Hickory's reviewed DNSSEC primitives.
That is an implementation detail. Hickory's configuration and authority server
are not part of the operator-facing design.

## Programs

`dnssec-keygen zone keyfile` creates one ECDSA P-256/SHA-256 key in PKCS#8 DER
form. It creates the key with mode 0600 and refuses to overwrite a path. The
only stdout output is the public `K` policy line described below.

`dnssec-sign [data [data.signed]]` reads tinydns data plus `dnssec` in the
current directory. It resolves no names and changes no live file. It writes a
complete signed tinydns snapshot to a sibling temporary file, syncs it, and
renames it over the requested output. It exits nonzero without changing the
last good output if any input, key, zone, or signature is invalid.

`dnssec-data [data [data.cdb]]` is the composition used for publication. It
runs the same signing transform and CDB compiler, writes a sibling temporary
CDB, syncs it, validates it by loading it, and renames it over `data.cdb`.
Keeping `dnssec-sign` separate makes signed text inspectable and lets an
operator compose it with other compilers.

`dnssec-ds dnssec-line` prints the DS record which must be installed at the
parent. It reads the referenced private key only to derive its public key and
does not modify any file.

`dnssec-check [data.signed]` performs structural and cryptographic checks and
prints one tab-separated status line per signed zone. Nagios, cron, systemd,
or a shell script can consume the same output. Exit status alone is sufficient
for supervision.

The utility names intentionally describe transformations rather than a suite
manager. Every stage can be invoked and tested independently.

## One-line policy

The file named `dnssec` contains comments, blank lines, and one line per key:

```text
Kexample.com:/etc/rgbdns/keys/example.com.pk8:13:1209600:86400:3600
```

Fields are colon-delimited and contain no whitespace:

```text
Kzone:keyfile:algorithm:validity:refresh:inception-skew
```

- `zone` is an absolute DNS name in rgbdns text form; a final dot is optional.
- `keyfile` is an absolute path to a PKCS#8 DER private key.
- `algorithm` is `13` (ECDSAP256SHA256). Unknown algorithms are rejected.
- `validity` is the RRSIG lifetime in seconds.
- `refresh` is the minimum remaining lifetime accepted by `dnssec-check`.
- `inception-skew` moves signature inception into the past to tolerate clock
  skew.

The initial release uses one combined signing key per zone. Multiple active
`K` lines for a zone are reserved for an explicit rollover release; accepting
them now would promise rollover semantics that do not yet exist. Bounds are
enforced for every duration. Key material never appears in `data`, `dnssec`,
logs, stdout, CDB metadata, or AXFR-only private records.

## Snapshot contract

The unsigned `data` file is source. The signed text and CDB are derived,
replaceable artifacts. For each configured zone, the signer:

1. requires exactly one SOA and a conventional authoritative zone;
2. rejects location-dependent records, time cutoffs, and pre-existing DNSSEC
   records in that zone;
3. requires every ANAME to have been materialized to ordinary A and AAAA data;
4. adds the apex DNSKEY;
5. builds an NSEC chain over authoritative names, excluding data beneath zone
   cuts;
6. signs every authoritative RRset, including NSEC and DNSKEY; and
7. emits canonical generic tinydns records for DNSKEY, NSEC, and RRSIG.

The SOA serial belongs to the input producer. Signing does not invent a second
serial sequence. A byte-identical input, policy, key, and explicit signing time
produce the same RRset content (ECDSA signature bytes need not be reproducible).

Every authoritative zone in one source snapshot must have exactly one `K`
line. This fail-closed rule prevents an accidentally omitted policy line from
silently publishing an insecure zone. Unscoped glue outside those zones passes
through unchanged. DNSSEC must not be enabled for a zone whose answer varies
by client location or time: one signed owner/type must always identify one
RRset.

## Serving contract

`tinydns` remains keyless. With EDNS DO=1 it attaches the stored RRSIG for every
answer and authoritative RRset, serves DNSKEY, returns signed NSEC proofs for
NODATA and NXDOMAIN, returns signed DS data or authenticated denial at a
delegation, and preserves wildcard proof semantics. With DO=0 it omits DNSSEC
records unless they were explicitly queried.

Truncation removes complete RRsets together with their signatures and proofs;
it never emits a partial signed RRset. TCP retries receive the complete answer.

`axfrdns` transfers DNSKEY, RRSIG, NSEC, and DS as ordinary records. A secondary
imports and serves those records without a key. The experimental RGA1 ANAME
extension remains independently negotiated and must never be inferred from a
private RR type.

## Composition with ANAME and ACME

A signed snapshot cannot contain answers synthesized after signing. The
publication pipeline is therefore explicit:

```text
source -> acme overlay -> aname materialize -> dnssec-sign -> tinydns-data -> rename
```

Each arrow is a program reading a stable input and producing a new sibling
file. Failure leaves the prior CDB live. The signer performs no recursive DNS
lookups.

For ACME, delegation of `_acme-challenge` to a small unsigned validation zone is
the simplest deployment. When an inline signed validation zone is required,
the update receiver writes its overlay atomically and invokes the configured
publication command synchronously; an update is acknowledged only after the
new signed CDB is durable. The command is replaceable and receives paths, not
private key bytes.

For ANAME, a separate materializer resolves targets, enforces the configured
TTL ceiling, writes ordinary address records, and records their earliest
expiry. The signing/publishing schedule must refresh before that expiry.
Secondaries receive the materialized records and their signatures; they do not
resolve the target independently for a signed zone.

## Key lifecycle

Key creation, activation, parent DS publication, rollover, revocation, and
retirement are distinct operator actions represented by files and policy
lines. Version 0.6.0 delivers safe single-key operation and DS derivation. A
later feature release will add a deliberately specified multi-key rollover
state machine; operators must not simulate rollover by silently swapping a key
file under an unchanged policy line.

Private keys are primary-only, mode 0600, and readable only by the offline
publication account. `tinydns`, `axfrdns`, and secondaries do not receive key
access. Backups and destruction are operator-visible file operations.

## Failure and observability

All programs log one line per failure to stderr and use stable nonzero exit
status. They do not retry indefinitely. `dnssec-check` reports the SOA serial,
key tag, earliest expiration, remaining lifetime, and status. Service timers
run the check frequently enough to alert before the configured refresh window.

The last-known-good signed CDB remains authoritative after resolution, signing,
validation, transfer, or disk failures. Expired signatures are treated as an
urgent operational fault, never repaired by online signing inside `tinydns`.
