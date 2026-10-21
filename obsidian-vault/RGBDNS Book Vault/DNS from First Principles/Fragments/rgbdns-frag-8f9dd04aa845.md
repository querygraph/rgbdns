---
type: "code-fragment"
fragment_id: "rgbdns-frag-8f9dd04aa845"
source_path: "docs/DNSSEC.md"
code_note: "DNS from First Principles/Code/docs/DNSSEC.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Create a key and policy"
kind: "heading"
start_line: 13
end_line: 49
---

# Create a key and policy

- Fragment ID: `rgbdns-frag-8f9dd04aa845`
- Source file: [[DNS from First Principles/Code/docs/DNSSEC.md.source|docs/DNSSEC.md]]
- Lines: 13-49
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-8f9dd04aa845", "codeNote": "DNS from First Principles/Code/docs/DNSSEC.md.source", "sourcePath": "docs/DNSSEC.md", "startLine": 13, "endLine": 49}
```

## Excerpt

<span id="rgbdns-frag-8f9dd04aa845" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8f9dd04aa845: heading Create a key and policy

````markdown
## Create a key and policy

Create the key as root on the primary. `rgbsec-keygen` refuses to overwrite a
file and creates it mode 0600:

```sh
sudo install -d -o root -g root -m 0700 /etc/rgbdns/keys
sudo rgbsec-keygen example.com /etc/rgbdns/keys/example.com.pk8 \
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
one disposition line in the policy for every authoritative zone in the source
file. `K` signs a zone; `Uzone` explicitly leaves one unsigned:

```text
Ulegacy.example.
```

The signer fails closed if a zone is missing, duplicated, or both `K` and `U`.
This permits a deliberate mixed signed/unsigned CDB without making omission
mean “unsigned.”

Keep a recoverable encrypted backup of the key. Key replacement is not an
implicit file operation: version 0.6.5 deliberately supports one active
combined signing key per zone and does not claim an automated multi-key
rollover state machine.

````
