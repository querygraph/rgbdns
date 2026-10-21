---
type: "code-fragment"
fragment_id: "rgbdns-frag-2b2eb12647f4"
source_path: "RESEND_SENDMAIL.md"
code_note: "DNS from First Principles/Code/RESEND_SENDMAIL.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Option A msmtp"
kind: "heading"
start_line: 91
end_line: 171
---

# Option A msmtp

- Fragment ID: `rgbdns-frag-2b2eb12647f4`
- Source file: [[DNS from First Principles/Code/RESEND_SENDMAIL.md.source|RESEND_SENDMAIL.md]]
- Lines: 91-171
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-2b2eb12647f4", "codeNote": "DNS from First Principles/Code/RESEND_SENDMAIL.md.source", "sourcePath": "RESEND_SENDMAIL.md", "startLine": 91, "endLine": 171}
```

## Excerpt

<span id="rgbdns-frag-2b2eb12647f4" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2b2eb12647f4: heading Option A msmtp

````markdown
## Option A: msmtp

### Why use msmtp

Use `msmtp` when the host only needs to submit outbound messages through a
smarthost. It is small, has a direct SMTP configuration, and avoids managing a
full MTA queue, daemon, hostname identity, and Sendmail macro configuration.
This is generally the simplest choice for one daily report.

### Install and configure

Install the client on Debian/Ubuntu:

```sh
sudo apt-get update
sudo apt-get install msmtp
```

Create the system configuration:

```sh
sudo vi /etc/msmtprc
```

Use the actual Resend API key in place of the placeholder:

```text
defaults
auth on
tls on
tls_trust_file /etc/ssl/certs/ca-certificates.crt

account resend
host smtp.resend.com
port 587
user resend
password re_YOUR_RESEND_API_KEY
from rgbdns@hurz.net

account default : resend
```

Protect the key:

```sh
sudo chmod 600 /etc/msmtprc
```

Point the report at `msmtp`:

```sh
sudo vi /etc/rgbdns/query-report.env
```

```text
REPORT_SENDMAIL=/usr/bin/msmtp
REPORT_FROM=rgbdns@hurz.net
```

The report already supplies `To:`, `From:`, `Subject:`, and the message body,
so `msmtp -t` can read the complete message from standard input.

### Test msmtp

Send a small test message without putting the API key on the command line:

```sh
printf 'From: rgbdns@hurz.net\nTo: deliverable@gmail.com\nSubject: Resend test\n\nSMTP test\n' |
  sudo /usr/bin/msmtp -t -v
```

Then run the report service:

```sh
sudo systemctl reset-failed rgbdns-query-report.service
sudo systemctl start rgbdns-query-report.service
sudo journalctl -u rgbdns-query-report.service -n 50 --no-pager
```

An authentication failure normally means the API key is wrong, revoked, or
was left as a placeholder. A `550` domain error means the `From:` domain is
````
