---
type: "code-fragment"
fragment_id: "rgbdns-frag-9c640d31e0fe"
source_path: "docs/DNSSEC.md"
code_note: "DNS from First Principles/Code/docs/DNSSEC.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Inspect the stages manually"
kind: "heading"
start_line: 50
end_line: 79
---

# Inspect the stages manually

- Fragment ID: `rgbdns-frag-9c640d31e0fe`
- Source file: [[DNS from First Principles/Code/docs/DNSSEC.md.source|docs/DNSSEC.md]]
- Lines: 50-79
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-9c640d31e0fe", "codeNote": "DNS from First Principles/Code/docs/DNSSEC.md.source", "sourcePath": "docs/DNSSEC.md", "startLine": 50, "endLine": 79}
```

## Excerpt

<span id="rgbdns-frag-9c640d31e0fe" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9c640d31e0fe: heading Inspect the stages manually

````markdown
## Inspect the stages manually

The individual transformations can be run without systemd:

```sh
acme-materialize data /etc/rgbdns/acme-update.conf \
  /var/lib/rgbdns/tinydns data.acme
aname-materialize data.acme data.materialized dnssec
ln -s /etc/rgbdns/dnssec dnssec
rgbsec-sign data.materialized data.signed
rgbsec-data data.materialized data.cdb
rgbsec-check data.cdb /etc/rgbdns/dnssec
rgbsec-ds 'Kexample.com.:/etc/rgbdns/keys/example.com.pk8:13:1209600:86400:3600'
```

With the third argument, `aname-materialize` resolves ANAME only in `K` zones
and preserves ANAME directives in `U` zones. The first three commands expose
each text stage for inspection. `rgbsec-data` is the shorter sign-and-compile
transform when the intermediate signed text is not needed. Both signing
commands read the one-line policy from `dnssec` in their working directory.
Every output is built beside its destination and renamed only after a successful
write. `rgbsec-check` cryptographically verifies every authoritative RRset,
checks the NSEC chain and validity interval, and emits:

```text
example.com.    SERIAL    KEYTAG    EARLIEST_EXPIRATION    SECONDS_LEFT    ok
```

The fields are tab-separated and suitable for cron or another supervisor.

````
