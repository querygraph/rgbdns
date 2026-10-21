---
type: "code-fragment"
fragment_id: "rgbdns-frag-c72a1830d4c0"
source_path: "RESEND_SENDMAIL.md"
code_note: "DNS from First Principles/Code/RESEND_SENDMAIL.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Resend prerequisites"
kind: "heading"
start_line: 24
end_line: 57
---

# Resend prerequisites

- Fragment ID: `rgbdns-frag-c72a1830d4c0`
- Source file: [[DNS from First Principles/Code/RESEND_SENDMAIL.md.source|RESEND_SENDMAIL.md]]
- Lines: 24-57
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-c72a1830d4c0", "codeNote": "DNS from First Principles/Code/RESEND_SENDMAIL.md.source", "sourcePath": "RESEND_SENDMAIL.md", "startLine": 24, "endLine": 57}
```

## Excerpt

<span id="rgbdns-frag-c72a1830d4c0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c72a1830d4c0: heading Resend prerequisites

````markdown
## Resend prerequisites

In Resend:

1. Add and verify the sender domain, for example `hurz.net`.
2. Publish every DNS record Resend provides. The DKIM record commonly has the
   name `resend._domainkey` under the domain.
3. Create a sending API key. Treat it as a password and never commit it.

The Resend SMTP settings are:

```text
Host:     smtp.resend.com
Port:     587
Security: STARTTLS
Username: resend
Password: the Resend API key
```

Resend also documents ports 465/2465 for implicit TLS and 25/2587 for
STARTTLS. Port 587 is used in the examples below.

Verify the DKIM record from the server or any machine with `dig`:

```sh
dig +short CNAME resend._domainkey.hurz.net
dig @1.1.1.1 +short CNAME resend._domainkey.hurz.net
dig @8.8.8.8 +short CNAME resend._domainkey.hurz.net
```

The returned CNAME target must exactly match the value shown in the Resend
dashboard. A DKIM record alone does not authorize a different `From:` domain;
the message sender must still use the verified domain.

````
