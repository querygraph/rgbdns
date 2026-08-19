# Optional authoritative DNSSEC

rgbdns 0.6.0 adds authoritative DNSSEC as an explicit offline publication
pipeline. It is disabled by default. If `/etc/rgbdns/dnssec.env` and the
working-directory `dnssec` policy are absent, `tinydns-data`, `tinydns`, ACME,
ANAME, setup, and secondary synchronization follow the existing
djbdns-compatible path.

The authoritative server never opens a private key. It only serves DNSKEY,
RRSIG, NSEC, and DS records already present in its CDB. Secondaries receive the
same finished records over ordinary AXFR and also need no key.

## Create a key and policy

Create the key as root on the primary. `dnssec-keygen` refuses to overwrite a
file and creates it mode 0600:

```sh
sudo install -d -o root -g root -m 0700 /etc/rgbdns/keys
sudo dnssec-keygen example.com /etc/rgbdns/keys/example.com.pk8 \
  | sudo tee /etc/rgbdns/dnssec
sudo chown root:rgbdns /etc/rgbdns/dnssec
sudo chmod 0640 /etc/rgbdns/dnssec
```

The output is one colon-delimited line:

```text
Kexample.com.:/etc/rgbdns/keys/example.com.pk8:13:1209600:86400:3600
```

The fields are zone, absolute key path, algorithm, signature validity, refresh
window, and inception skew. Algorithm 13 is ECDSA P-256/SHA-256. Put exactly
one `K` line in the policy for every authoritative zone in the source file.
The signer fails closed if a zone is missing or duplicated.

Keep a recoverable encrypted backup of the key. Key replacement is not an
implicit file operation: version 0.6.0 deliberately supports one active
combined signing key per zone and does not claim an automated multi-key
rollover state machine.

## Inspect the stages manually

The individual transformations can be run without systemd:

```sh
acme-materialize data /etc/rgbdns/acme-update.conf \
  /var/lib/rgbdns/tinydns data.acme
aname-materialize data.acme data.materialized
ln -s /etc/rgbdns/dnssec dnssec
dnssec-sign data.materialized data.signed
dnssec-data data.materialized data.cdb
dnssec-check data.cdb /etc/rgbdns/dnssec
dnssec-ds 'Kexample.com.:/etc/rgbdns/keys/example.com.pk8:13:1209600:86400:3600'
```

The first three commands expose each text stage for inspection. `dnssec-data`
is the shorter sign-and-compile transform when the intermediate signed text is
not needed. Both signing commands read the one-line policy from `dnssec` in
their working directory.
Every output is built beside its destination and renamed only after a successful
write. `dnssec-check` cryptographically verifies every authoritative RRset,
checks the NSEC chain and validity interval, and emits:

```text
example.com.    SERIAL    KEYTAG    EARLIEST_EXPIRATION    SECONDS_LEFT    ok
```

The fields are tab-separated and suitable for cron or another supervisor.

## Enable packaged publication

Pass the policy while configuring a primary:

```sh
sudo rgbdns-setup primary \
  --data /path/to/rgbdns.data \
  --dnssec-policy /etc/rgbdns/dnssec
```

This creates `/etc/rgbdns/dnssec.env`, performs the first publication as root,
and enables two timers. `rgbdns-dnssec-publish.timer` refreshes materialized
ANAME data and signatures every 12 hours. `rgbdns-dnssec-check.timer` verifies
the active CDB hourly as the unprivileged `rgbdns` account. A failed resolver,
signer, compiler, or checker leaves the prior CDB active.

The root-only publisher reads the private key. The `rgbdns` service account
can read the public policy and active CDB but cannot traverse
`/etc/rgbdns/keys`. Removing `/etc/rgbdns/dnssec.env` returns `compile-zone` to
the original `tinydns-data` path; use `rgbdns-setup` to make role changes so
timer state remains consistent.

## ANAME and ACME

A signed answer cannot be synthesized after signing. `aname-materialize`
therefore resolves every private `Aowner:target:ttl-cap` directive first and
writes ordinary A/AAAA records with capped TTLs. The scheduled publisher
refreshes them before signatures expire. Signed AXFR contains the materialized
addresses and signatures, not a requirement for the secondary to resolve the
target.

For ACME, delegation of `_acme-challenge` to a small unsigned validation zone
is simplest. Inline updates to a signed zone require the packaged synchronous
publisher. `rgbdns-setup` sets `ACME_PUBLISH_COMMAND` only when both
`--acme-update-config` and `--dnssec-policy` are present. The UPDATE receiver
persists the overlay, invokes the bounded publication command, reloads and
validates the signed CDB, and returns success only then. Without the hook, a
signed zone refuses ACME update startup rather than publishing unsigned TXT.

## Parent DS and activation

Print the DS line with `dnssec-ds`, then install the exact key tag, algorithm
13, digest type 2, and digest at the parent. Do not publish the DS until every
delegated authority answers DNSKEY and signed positive and negative queries.

Useful checks are:

```sh
dig @127.0.0.1 example.com SOA +dnssec
dig @127.0.0.1 example.com DNSKEY +dnssec
dig @127.0.0.1 does-not-exist.example.com A +dnssec
dig @127.0.0.1 child.example.com DS +dnssec
delv example.com
```

After parent publication, monitor from more than one validating network. To
disable DNSSEC safely, remove the parent DS first, wait through its TTL and
negative caches, and only then remove local policy and republish unsigned data.

## Constraints

- Location-dependent (`%`) or TAI64 activation/expiration data cannot be
  signed because one owner/type must identify one stable RRset.
- ANAME must be materialized before signing.
- Pre-existing DNSKEY, RRSIG, NSEC, or NSEC3 records in source are rejected.
- NSEC is intentionally used instead of NSEC3: it is smaller, simpler, and
  avoids iteration and opt-out complexity. Zone contents are enumerable.
- Direct public-network DNSSEC validation remains an opt-in test.

The exact command and file contract is specified in
[`DNSSEC-DESIGN.md`](DNSSEC-DESIGN.md).
