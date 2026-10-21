---
type: "code-fragment"
fragment_id: "rgbdns-frag-ce4850ac1142"
source_path: "RESEND_SENDMAIL.md"
code_note: "DNS from First Principles/Code/RESEND_SENDMAIL.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "etc aliases"
kind: "heading"
start_line: 465
end_line: 545
---

# etc aliases

- Fragment ID: `rgbdns-frag-ce4850ac1142`
- Source file: [[DNS from First Principles/Code/RESEND_SENDMAIL.md.source|RESEND_SENDMAIL.md]]
- Lines: 465-545
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-ce4850ac1142", "codeNote": "DNS from First Principles/Code/RESEND_SENDMAIL.md.source", "sourcePath": "RESEND_SENDMAIL.md", "startLine": 465, "endLine": 545}
```

## Excerpt

<span id="rgbdns-frag-ce4850ac1142" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ce4850ac1142: heading etc aliases

````markdown
# /etc/aliases
dick: deliverable@gmail.com
```

After editing `/etc/aliases`, compile the aliases and reload the services:

```sh
sudo newaliases
sudo make -C /etc/mail
sudo systemctl restart sendmail
sudo systemctl start rgbdns-data-import.service
```

The Sendmail listener must be reachable on TCP port 25. Confirm locally with
`ss -ltnp`; the listener should include `0.0.0.0:25`. Also allow inbound TCP
25 in the AWS security group. A local forwarding test only proves alias
expansion and outbound relay; it does not prove that an external sender can
reach the server.

Verify DNS from both the authoritative server and public resolvers:

```sh
dig @a.ns.cron.sh MX hurz.net +short
dig @1.1.1.1 MX hurz.net +short
dig @a.ns.cron.sh A mail.hurz.net +short
```

Public resolvers may continue returning the previous MX until its TTL expires.

Test the alias without verbose SMTP output:

```sh
printf 'From: test@hurz.net\nTo: dick@hurz.net\nSubject: forwarding test\n\nTest.\n' |
  sudo /usr/sbin/sendmail -t
sudo journalctl -u sendmail --since "5 minutes ago" -l --no-pager
sudo mailq
```

The successful Gmail-relay result observed in production was `relay=smtp.gmail.com`,
`dsn=2.0.0`, `stat=Sent`, followed by an empty Sendmail queue. The successful
TLS result was `verify=OK` with TLS 1.3 and an AES-256 cipher.

### Operational tradeoff

Self-hosting is appropriate when you already operate an MTA and want complete
control over inbound mail. For one forwarding address, Cloudflare Email
Routing is usually safer and simpler: it receives mail for `hurz.net`, verifies
`deliverable@gmail.com` as the destination, and manages the forwarding path.
It requires Cloudflare DNS and changes the domain's MX records, so it must not
be enabled alongside another inbound mail provider.

### Use Gmail instead of Resend with native Sendmail

Native Sendmail can use Gmail as its authenticated smarthost. Create a Google
App Password first; do not put the normal Gmail password in the auth map.

Edit the auth source:

```sh
sudo vi /etc/mail/authinfo
```

```text
AuthInfo:smtp.gmail.com "U:deliverable@gmail.com" "P:YOUR_16_DIGIT_APP_PASSWORD" "M:PLAIN"
```

Compile it:

```sh
sudo chmod 600 /etc/mail/authinfo
sudo sh -c 'makemap hash /etc/mail/authinfo < /etc/mail/authinfo'
```

Edit the Sendmail macros:

```sh
sudo vi /etc/mail/sendmail.mc
```

Use Gmail instead of the Resend definitions:

````
