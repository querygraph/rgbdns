---
type: "code-fragment"
fragment_id: "rgbdns-frag-297bb68d9312"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Minimal signed primary"
kind: "heading"
start_line: 763
end_line: 830
---

# Minimal signed primary

- Fragment ID: `rgbdns-frag-297bb68d9312`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 763-830
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-297bb68d9312", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 763, "endLine": 830}
```

## Excerpt

<span id="rgbdns-frag-297bb68d9312" class="rgbdns-fragment-target"></span>
### rgbdns-frag-297bb68d9312: heading Minimal signed primary

````markdown
## Minimal signed primary

Begin with ordinary tinydns source. The signer preserves the SOA serial, so the
source producer must increment it when the signed publication first becomes
visible:

```text
Zexample.com:a.ns.example.com:hostmaster.example.com:2026082101:16384:2048:1048576:2560:3600
&example.com:192.0.2.53:a.ns.example.com:3600
+example.com:192.0.2.80:3600
+www.example.com:192.0.2.80:3600
```

Create the key only on the primary. The command refuses to overwrite an
existing file and creates the PKCS#8 key with mode 0600:

```sh
sudo install -d -o root -g root -m 0700 /etc/rgbdns/keys
sudo rgbsec-keygen example.com \
  /etc/rgbdns/keys/example.com.pk8 \
  | sudo tee /etc/rgbdns/dnssec
sudo chown root:rgbdns /etc/rgbdns/dnssec
sudo chmod 0640 /etc/rgbdns/dnssec
```

Keep an encrypted, recoverable copy of the key outside the server. The public
policy may be readable by the checker, but the key directory remains
root-only. The authority process and the secondary never need access.

Configure the packaged primary with the complete existing role options plus
the policy:

```sh
sudo rgbdns-setup primary \
  --data /srv/dns/rgbdns.data \
  --data-drop /srv/dns/rgbdns.data \
  --data-drop-owner dns-publisher \
  --listen-ip 0.0.0.0 \
  --allow-nets 10.0.2.10/32 \
  --dnssec-policy /etc/rgbdns/dnssec
```

Setup performs the first privileged publication and enables two supervised
jobs. `rgbdns-dnssec-publish.timer` refreshes materialized data and signatures
every twelve hours. `rgbdns-dnssec-check.timer` checks the active CDB hourly.
The root publisher can read the key; the checker runs as `rgbdns` and verifies
only public state.

The same stages can be inspected manually in a scratch directory:

```sh
ln -s /etc/rgbdns/dnssec dnssec
rgbsec-sign data data.signed
rgbsec-data data data.cdb
rgbsec-check data.cdb /etc/rgbdns/dnssec
rgbsec-ds 'Kexample.com.:/etc/rgbdns/keys/example.com.pk8:13:1209600:86400:3600'
```

`rgbsec-check` emits one tab-separated line per `K` zone:

```text
example.com.  2026082101  53856  20260904023527  1204490  ok
```

The exact times and key tag vary. The important operational contract is the
exit status and final `ok`: every authoritative RRset verifies, the NSEC cycle
is complete, and the earliest signature remains outside the refresh window.

````
