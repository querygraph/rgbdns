---
type: "code-fragment"
fragment_id: "rgbdns-frag-ef1499ee3dcf"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "6. Enable ACME DNS-01 updates on a primary"
kind: "heading"
start_line: 181
end_line: 236
---

# 6. Enable ACME DNS-01 updates on a primary

- Fragment ID: `rgbdns-frag-ef1499ee3dcf`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 181-236
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-ef1499ee3dcf", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 181, "endLine": 236}
```

## Excerpt

<span id="rgbdns-frag-ef1499ee3dcf" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ef1499ee3dcf: heading 6. Enable ACME DNS-01 updates on a primary

````markdown
## 6. Enable ACME DNS-01 updates on a primary

This optional feature lets Certbot or another RFC 2136 client create only the
TXT records required for certificate validation. It does not issue or install
certificates. Generate a different 32-byte secret for each certificate host:

```sh
openssl rand -base64 32
```

Add a root-owned policy to `/etc/rgbdns/acme-update.conf`:

```text
certbot-fieldnotes. hmac-sha256. BASE64_SECRET fieldnotes.es. _acme-challenge. 60
```

The file must remain mode 0640 and group `rgbdns`. Re-run primary setup with
the complete existing options plus:

```text
--acme-update-config /etc/rgbdns/acme-update.conf
```

Configure Certbot's RFC 2136 plugin with:

```ini
dns_rfc2136_server = 52.10.53.234
dns_rfc2136_port = 53
dns_rfc2136_name = certbot-fieldnotes.
dns_rfc2136_secret = BASE64_SECRET
dns_rfc2136_algorithm = HMAC-SHA256
```

Keep Certbot's credentials file mode 0600. Use the staging CA for the first
renewal test and allow enough propagation time for all authoritative
secondaries. Confirm publication and transfer:

```sh
dig @52.10.53.234 _acme-challenge.fieldnotes.es TXT +tcp
dig @172.31.60.189 fieldnotes.es AXFR |
  grep _acme-challenge.fieldnotes.es
```

The administrative client exercises the identical authenticated update path:

```sh
sudo rgbdns-acme present --zone fieldnotes.es \
  --name _acme-challenge.fieldnotes.es --value staging-token
sudo rgbdns-acme cleanup --zone fieldnotes.es \
  --name _acme-challenge.fieldnotes.es --value staging-token
```

Do not enable ACME UPDATE on a secondary. The primary advances its SOA serial
and standard AXFR distributes the public TXT state. Canonical data imports are
validated together with the durable overlay before activation.

````
