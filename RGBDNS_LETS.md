# Automated ACME DNS Validation in rgbdns

## Status

This document specifies the rgbdns 0.4.0 ACME DNS-01 update feature. It is the
implementation contract for authenticated dynamic TXT updates, durable ACME
state, atomic publication, zone transfer, packaging, and interoperability with
RFC 2136-capable ACME clients such as Certbot.

rgbdns does not implement the ACME protocol, request certificates, store
private certificate keys, or terminate TLS. An ACME client performs those
jobs. rgbdns provides the narrowly scoped authoritative DNS mutation needed to
answer DNS-01 challenges.

## Goals

- Accept standards-based RFC 2136 DNS UPDATE messages from existing ACME
  clients.
- Authenticate every accepted update with a TSIG HMAC-SHA256 key.
- Authorize a key only for configured `_acme-challenge` owners in configured
  primary zones.
- Add and remove individual TXT values without destroying concurrent
  challenges at the same owner.
- Make a successful update visible on the primary before returning success.
- Persist accepted state across daemon restart and canonical zone imports.
- Include dynamic challenge records and the advanced SOA serial in AXFR so
  secondaries converge normally.
- Preserve the existing atomic `rgbdns.data` import and last-known-good
  behavior.
- Bound packet, state, credential, name, value, and concurrency resource use.

## Non-goals

- General-purpose dynamic DNS.
- UPDATE of A, AAAA, ANAME, CNAME, NS, SOA, MX, CAA, DNSSEC, or arbitrary
  record types.
- UPDATE forwarding by a secondary.
- Certificate issuance, renewal scheduling, deployment, or TLS service.
- TSIG algorithms other than HMAC-SHA256 in 0.4.0.
- DNS UPDATE over UDP in 0.4.0. TCP avoids truncation and amplification and is
  supported by the intended RFC 2136 clients.
- Arbitrary RFC 2136 prerequisites. The limited prerequisite forms required by
  common clients are validated; unsupported forms receive NOTIMP.

## Operator model

The primary remains the only writable authority. Canonical configuration is
still delivered as `rgbdns.data`. ACME state is stored separately under the
tinydns state directory:

```text
/var/lib/rgbdns/tinydns/data                 canonical activated text
/var/lib/rgbdns/tinydns/data.cdb             canonical compiled database
/var/lib/rgbdns/tinydns/acme-overlay.data    durable generated TXT overlay
/var/lib/rgbdns/tinydns/acme-serials         durable per-zone SOA serials
/var/lib/rgbdns/tinydns/acme-update.lock     cross-process publication lock
```

The generated files are rgbdns state, not conffiles. Operators must not edit
them. Challenge credentials live in `/etc/rgbdns/acme-update.conf`, owned by
root, group `rgbdns`, mode 0640. Packages preserve this file during upgrades.

The daemon constructs and serves one logical snapshot from canonical data plus
the ACME overlay. Each accepted update performs this transaction:

1. Authenticate and authorize the complete request without changing state.
2. Lock the update state and reload the durable overlay.
3. Apply additions and value-specific deletions to an in-memory candidate.
4. Advance affected zone serials monotonically.
5. Write candidate overlay and serial files beside their destinations, `fsync`
   each file, rename them atomically, and `fsync` the state directory.
6. Build and validate a new logical `Zone` snapshot.
7. Atomically replace the daemon's shared snapshot.
8. Return a signed NOERROR response.

No successful response may precede durable state and live publication. A
failure leaves both the old durable files and the old live snapshot usable.
Canonical imports take the same lock, validate the canonical candidate merged
with the current overlay, activate canonical files, and notify/restart the
daemon without deleting ACME state.

## Configuration

ACME updates are disabled when `ACME_UPDATE_CONFIG` is absent or empty. A
primary enables them with:

```text
ACME_UPDATE_CONFIG=/etc/rgbdns/acme-update.conf
ACME_STATE_DIR=/var/lib/rgbdns/tinydns
```

The credential file uses one tab-separated policy per non-comment line:

```text
# key-name  algorithm     base64-secret  zone                  owner-prefix       ttl
certbot-chiefscientist. hmac-sha256. SECRET chiefscientist.org. _acme-challenge. 60
```

Fields have these meanings:

- `key-name`: absolute TSIG owner, compared case-insensitively.
- `algorithm`: exactly `hmac-sha256.` (the common aliases are accepted on the
  wire).
- `base64-secret`: decoded key bytes, at least 16 and at most 64 bytes.
- `zone`: absolute primary zone authorized for this key.
- `owner-prefix`: exactly `_acme-challenge.` in 0.4.0. It authorizes both
  `_acme-challenge.<zone>` and `_acme-challenge.<validated-name>.<zone>`.
- `ttl`: positive TTL from 30 through 300 seconds. The server replaces client
  TTLs with this policy TTL.

Repeated key names are rejected. Configuration, state, and canonical data are
bounded. The implementation rejects more than 256 keys, 256 managed zones,
128 live values per owner, 4096 total live challenge values, 255 bytes per TXT
RDATA, or 64 KiB of generated overlay state.

## DNS UPDATE profile

The message opcode is UPDATE (5). The Zone section contains exactly one SOA
question in class IN. The Update section may contain only these operations:

- Add one TXT value: class IN, type TXT, nonzero TTL, one or more character
  strings whose concatenation is at most 255 bytes.
- Delete one TXT value: class NONE, type TXT, TTL zero, matching RDATA.
- Delete the TXT RRset: class ANY, type TXT, TTL zero, empty RDATA. This is
  accepted for client cleanup but affects only the authorized owner.

Multiple operations are atomic. Names must be inside the Zone section zone and
must match the authenticated key's policy. All other record types, classes,
zone shapes, meta-records, and delete-name operations are refused.

The first implementation accepts an empty prerequisite section. It also
accepts the RFC 2136 value-independent "RRset exists" and "RRset does not
exist" TXT prerequisites at authorized owners. Unsupported prerequisites
return NOTIMP; false prerequisites return YXRRSET or NXRRSET as appropriate.

Only TCP UPDATE is processed. UPDATE received over UDP returns REFUSED. An
UPDATE sent to a secondary returns NOTAUTH.

## TSIG profile

The request must contain exactly one TSIG RR as the last Additional record.
TSIG is decoded without interpreting its embedded names as ordinary opaque
RDATA. The server verifies:

- configured key name and HMAC-SHA256 algorithm;
- original ID equal to the DNS message ID;
- signed time within the request fudge, with a server maximum fudge of 300
  seconds;
- MAC over the exact RFC 2845 request input using constant-time comparison;
- MAC length of 32 bytes;
- no trailing or duplicate TSIG.

Successful and DNS-error responses are TSIG-signed using the request MAC.
Authentication failures use NOTAUTH with TSIG error BADKEY, BADSIG, or BADTIME
when a safe signed response is possible. Malformed messages receive FORMERR.
Secrets are never logged. Logs identify only the key name, zone, owner, action,
result, and client address.

## Zone data and serials

The durable overlay is generated tinydns text containing escaped TXT lines:

```text
'_acme-challenge.chiefscientist.org:token:60
```

Generated values use tinydns octal escapes where required, are sorted by owner
and value, and are parsed by the normal zone parser before activation. Dynamic
data may coexist only with TXT records at the owner; a canonical CNAME or other
conflicting data causes publication to fail safely.

For every affected primary zone the update serial becomes:

```text
max(previous_dynamic_serial + 1, canonical_serial + 1, current_unix_time)
```

with RFC 1982 wrapping rules. The logical zone replaces that zone's SOA serial
in authoritative answers and AXFR without rewriting the operator's canonical
SOA line. Overlay TXT records appear in ordinary lookup and AXFR. Standard and
rgbdns secondaries therefore receive the same public DNS-01 state; no private
transfer extension is involved.

## Canonical import interaction

`rgbdns-import-data` must preserve the operator's existing atomic delivery
contract. Its staging compilation validates canonical data alone and then the
canonical-plus-overlay logical snapshot. It holds the shared update lock only
for final revalidation and activation. If merge validation fails, neither
canonical files nor ACME state changes.

A canonical import may add a permanent TXT record at an ACME owner. It may not
add a CNAME or non-TXT data at an owner with live challenge state. Operators
must first allow challenge cleanup or explicitly clear the overlay with the
administrative command.

## Administrative command

`rgbdns-acme` provides local, root/operator-controlled state management and
Certbot manual-hook compatibility:

```text
rgbdns-acme present --zone ZONE --name OWNER --value VALUE
rgbdns-acme cleanup --zone ZONE --name OWNER --value VALUE
rgbdns-acme clear --zone ZONE [--name OWNER]
rgbdns-acme list [--zone ZONE]
```

`present` and `cleanup` use the same policy validation, lock, transaction, and
publication mechanism as network UPDATE. They do not require or expose a TSIG
secret. Mutating commands require root or the configured service account.
Machine-readable output is available with `--json`. The command returns only
after the local primary serves the requested state.

## Certbot interoperability

An operator creates a separate key per certificate host or administrative
boundary and configures Certbot's RFC 2136 plugin with the primary address,
port 53, key name, HMAC-SHA256 secret, and algorithm. The credentials file must
not be world-readable. Renewal uses DNS-01 and an explicit propagation delay
long enough for every authoritative secondary.

Operators may instead delegate `_acme-challenge` with CNAME or NS records to a
small rgbdns validation zone. This limits update authority and follows the
DNS-01 delegation behavior documented by Let's Encrypt.

The integration test uses an RFC 2136 client to add a staging-shaped token,
queries UDP and TCP authority, transfers the zone, restarts the daemon, imports
new canonical data, removes only that token, and confirms unrelated/concurrent
TXT values remain.

## Security properties

- Network updates are disabled by default and fail closed on bad config.
- There is no unauthenticated source-address authorization fallback.
- A compromised certificate host receives authority only over configured ACME
  TXT owners, not the zone.
- TCP-only operation and strict size limits constrain amplification and memory
  use.
- Replay resistance comes from TSIG signed time and bounded fudge. The server
  also keeps a bounded recent-MAC cache for the acceptance window and rejects
  an identical authenticated request twice.
- File permissions, atomic replacement, durable writes, and no-follow opens
  protect local state.
- Logs do not contain TSIG secrets or ACME token values.
- Failed authentication, authorization, prerequisites, persistence, merge, or
  publication cannot partially apply a multi-operation update.

## Packaging and service integration

Debian and RPM packages install:

- `/etc/rgbdns/acme-update.conf` as a preserved, mode-0640 example conffile;
- the `rgbdns-acme` binary and manual page;
- updated `tinydns`, `rgbdns-setup`, and import manual pages;
- service defaults with ACME disabled;
- tmpfiles/setup rules for state ownership where required.

`rgbdns-setup --acme-update-config PATH` enables the feature on a primary and
rejects it for a secondary. Package upgrades preserve operator configuration.
Removal does not delete durable ACME state or credentials.

## Validation matrix

The feature is not complete until tests cover:

- RFC 2136 message decoding and response codes;
- RFC 2845 known-answer TSIG vectors and response signing;
- malformed, missing, duplicate, misplaced, expired, future, bad-key, bad-MAC,
  replayed, and unsupported-algorithm requests;
- zone, owner, type, class, TTL, TXT length, state-size, and count policy;
- atomic multi-value add, value delete, RRset delete, and prerequisites;
- simultaneous wildcard/non-wildcard values at one owner;
- concurrent update serialization;
- durable restart and canonical re-import;
- SOA serial monotonicity and wrap behavior;
- authoritative UDP/TCP lookup and integrated AXFR visibility;
- secondary refusal and primary-only configuration;
- package verification on Debian and RPM;
- Certbot-compatible RFC 2136 configuration against a local integration
  fixture and a documented opt-in Let's Encrypt staging test.

The repository validation commands remain:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
packaging/tests/test-secondary-sync.sh
make -C ietf check
git diff --check
```

The direct public-network Let's Encrypt staging test is opt-in because it
requires delegated public DNS and consumes external service capacity.
