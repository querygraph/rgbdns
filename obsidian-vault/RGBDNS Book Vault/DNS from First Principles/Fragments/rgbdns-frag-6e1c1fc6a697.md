---
type: "code-fragment"
fragment_id: "rgbdns-frag-6e1c1fc6a697"
source_path: "RESEND_SENDMAIL.md"
code_note: "DNS from First Principles/Code/RESEND_SENDMAIL.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Option B native Sendmail"
kind: "heading"
start_line: 225
end_line: 305
---

# Option B native Sendmail

- Fragment ID: `rgbdns-frag-6e1c1fc6a697`
- Source file: [[DNS from First Principles/Code/RESEND_SENDMAIL.md.source|RESEND_SENDMAIL.md]]
- Lines: 225-305
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-6e1c1fc6a697", "codeNote": "DNS from First Principles/Code/RESEND_SENDMAIL.md.source", "sourcePath": "RESEND_SENDMAIL.md", "startLine": 225, "endLine": 305}
```

## Excerpt

<span id="rgbdns-frag-6e1c1fc6a697" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6e1c1fc6a697: heading Option B native Sendmail

````markdown
## Option B: native Sendmail

### Why use Sendmail

Use native Sendmail when the host already depends on it, other local services
submit mail through `/usr/sbin/sendmail`, or you need a conventional MTA queue
and daemon. It can relay through Resend, but its configuration is more involved:
SASL support, an auth database, Sendmail macros, TLS trust, daemon reloads, and
hostname identity all matter.

Installing Sendmail alone is not enough. Without a smarthost, Sendmail tries
to deliver directly to the recipient's MX. Gmail commonly rejects that direct
mail with:

```text
550 5.7.26 Your email has been blocked because the sender is unauthenticated
```

### Install SASL support

On Debian/Ubuntu:

```sh
sudo apt-get update
sudo apt-get install sasl2-bin libsasl2-modules
sudo /usr/share/sendmail/update_auth
```

### Create the Resend auth map

Edit the source auth file:

```sh
sudo vi /etc/mail/authinfo
```

Add one line with the real API key:

```text
AuthInfo:smtp.resend.com "U:resend" "P:re_YOUR_RESEND_API_KEY" "M:PLAIN"
```

Protect and compile it. The `sh -c` is important because shell redirection
happens before `sudo` when written directly as `sudo makemap ... < file`:

```sh
sudo chmod 600 /etc/mail/authinfo
sudo sh -c 'makemap hash /etc/mail/authinfo < /etc/mail/authinfo'
```

Verify both files exist:

```sh
sudo ls -l /etc/mail/authinfo /etc/mail/authinfo.db
```

### Configure the Sendmail smarthost

Edit the macro configuration:

```sh
sudo vi /etc/mail/sendmail.mc
```

Add these definitions before the `MAILER` lines:

```m4
define(`SMART_HOST', `smtp.resend.com')dnl
define(`RELAY_MAILER_ARGS', `TCP $h 587')dnl
define(`ESMTP_MAILER_ARGS', `TCP $h 587')dnl
FEATURE(`authinfo', `hash -o /etc/mail/authinfo.db')dnl
define(`confAUTH_MECHANISMS', `PLAIN LOGIN')dnl
define(`confCACERT', `/etc/ssl/certs/ca-certificates.crt')dnl
```

Rebuild and restart Sendmail:

```sh
sudo make -C /etc/mail
sudo systemctl restart sendmail
```
````
