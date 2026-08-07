# Resend mail delivery for rgbdns reports

This runbook configures the daily `rgbdns-query-report.service` to deliver
through [Resend SMTP](https://resend.com/docs/send-with-smtp). It covers two
sendmail-compatible transports:

1. `msmtp`, a small SMTP client; and
2. Sendmail itself, configured as an authenticated Resend smarthost.

The report script does not speak Resend's HTTP API directly. It writes a
complete message to the executable named by `REPORT_SENDMAIL` and invokes it
with `-t`:

```text
# /etc/rgbdns/query-report.env
REPORT_TO=deliverable@gmail.com
REPORT_FROM=rgbdns@hutz.net
REPORT_SENDMAIL=/usr/sbin/sendmail
```

`REPORT_FROM` must use a domain verified in Resend. The hostname of the DNS
server, such as `a.ns.cron.sh`, does not need to be the mail sender domain.

## Resend prerequisites

In Resend:

1. Add and verify the sender domain, for example `hutz.net`.
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
dig +short CNAME resend._domainkey.hutz.net
dig @1.1.1.1 +short CNAME resend._domainkey.hutz.net
dig @8.8.8.8 +short CNAME resend._domainkey.hutz.net
```

The returned CNAME target must exactly match the value shown in the Resend
dashboard. A DKIM record alone does not authorize a different `From:` domain;
the message sender must still use the verified domain.

## Configure the rgbdns report

On the primary only, edit the report configuration:

```sh
sudo vi /etc/rgbdns/query-report.env
```

Use a verified sender address:

```text
REPORT_TO=deliverable@gmail.com
REPORT_FROM=rgbdns@hutz.net
```

The report timer is intentionally opt-in and should normally run only on the
primary, not on the secondary:

```sh
sudo systemctl daemon-reload
sudo systemctl enable --now rgbdns-query-report.timer
systemctl list-timers --all rgbdns-query-report.timer
```

The timer is not the mail transport. It only starts the report service. A
manual run sends the preceding local calendar day's report immediately:

```sh
sudo systemctl reset-failed rgbdns-query-report.service
sudo systemctl start rgbdns-query-report.service
sudo systemctl status rgbdns-query-report.service --no-pager
```

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
from rgbdns@hutz.net

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
REPORT_FROM=rgbdns@hutz.net
```

The report already supplies `To:`, `From:`, `Subject:`, and the message body,
so `msmtp -t` can read the complete message from standard input.

### Test msmtp

Send a small test message without putting the API key on the command line:

```sh
printf 'From: rgbdns@hutz.net\nTo: deliverable@gmail.com\nSubject: Resend test\n\nSMTP test\n' |
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
not the verified Resend domain.

### Use Gmail instead of Resend with msmtp

`msmtp` can submit through Gmail directly. Gmail's SMTP submission endpoint is
`smtp.gmail.com` on port 587 with STARTTLS and authentication. Use a Google App
Password, not the normal account password. App Passwords require 2-Step
Verification and may be unavailable for some managed accounts. See Google's
[SMTP settings](https://support.google.com/mail/answer/7104828) and
[App Password guidance](https://support.google.com/accounts/answer/185833).

Create an App Password for the Gmail account that will send the report, then
edit:

```sh
sudo vi /etc/msmtprc
```

```text
defaults
auth on
tls on
tls_trust_file /etc/ssl/certs/ca-certificates.crt

account gmail
host smtp.gmail.com
port 587
user deliverable@gmail.com
password YOUR_16_DIGIT_APP_PASSWORD
from deliverable@gmail.com

account default : gmail
```

Protect the file and select `msmtp` for the report:

```sh
sudo chmod 600 /etc/msmtprc
sudo vi /etc/rgbdns/query-report.env
```

```text
REPORT_SENDMAIL=/usr/bin/msmtp
REPORT_FROM=deliverable@gmail.com
```

Test through Gmail:

```sh
printf 'From: deliverable@gmail.com\nTo: deliverable@gmail.com\nSubject: Gmail SMTP test\n\nSMTP test\n' |
  sudo /usr/bin/msmtp -t -v
```

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

The generated configuration must contain the smarthost:

```sh
sudo grep '^DS' /etc/mail/sendmail.cf
```

Expected output:

```text
DSsmtp.resend.com
```

If it still says `DS` with no host, or the delivery logs show
`gmail-smtp-in.google.com`, Sendmail is still delivering directly and the
smarthost configuration was not applied.

### Ensure Sendmail has a resolvable hostname

Sendmail may warn about an unqualified AWS hostname. Set the primary's local
hostname and map it locally:

```sh
sudo hostnamectl set-hostname a.ns.cron.sh
sudo vi /etc/hosts
```

Ensure this line exists:

```text
172.31.60.189 a.ns.cron.sh a
```

Verify:

```sh
hostname --fqdn
getent hosts a.ns.cron.sh
```

The hostname warning is separate from Resend authentication, but fixing it
gives Sendmail a stable local identity.

Point rgbdns back to native Sendmail:

```sh
sudo vi /etc/rgbdns/query-report.env
```

```text
REPORT_SENDMAIL=/usr/sbin/sendmail
REPORT_FROM=rgbdns@hutz.net
```

### Test native Sendmail

```sh
printf 'From: rgbdns@hutz.net\nTo: deliverable@gmail.com\nSubject: Resend test\n\nSMTP test\n' |
  sudo /usr/sbin/sendmail -v -t
```

Then run the report and inspect the MTA logs:

```sh
sudo systemctl reset-failed rgbdns-query-report.service
sudo systemctl start rgbdns-query-report.service
sudo journalctl -u sendmail --since "5 minutes ago" --no-pager
sudo mailq
```

Successful Sendmail delivery should include:

```text
relay=smtp.resend.com
dsn=2.0.0
stat=Sent
```

The queue should be empty after successful delivery. If the log instead shows
`relay=gmail-smtp-in.google.com`, Sendmail is bypassing Resend. If it shows
`535 Authentication credentials invalid`, check the API key and auth map. If
it shows `550 ... domain is not verified`, use a verified domain in both
`REPORT_FROM` and the Sendmail/msmtp sender configuration.

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

```m4
define(`SMART_HOST', `smtp.gmail.com')dnl
define(`RELAY_MAILER_ARGS', `TCP $h 587')dnl
define(`ESMTP_MAILER_ARGS', `TCP $h 587')dnl
FEATURE(`authinfo', `hash -o /etc/mail/authinfo.db')dnl
define(`confAUTH_MECHANISMS', `PLAIN LOGIN')dnl
define(`confCACERT', `/etc/ssl/certs/ca-certificates.crt')dnl
```

Rebuild and restart:

```sh
sudo make -C /etc/mail
sudo systemctl restart sendmail
```

Use the Gmail account as the report sender:

```sh
sudo vi /etc/rgbdns/query-report.env
```

```text
REPORT_SENDMAIL=/usr/sbin/sendmail
REPORT_FROM=deliverable@gmail.com
```

Verify and test:

```sh
sudo grep '^DS' /etc/mail/sendmail.cf
printf 'From: deliverable@gmail.com\nTo: deliverable@gmail.com\nSubject: Gmail SMTP test\n\nSMTP test\n' |
  sudo /usr/sbin/sendmail -v -t
```

The Sendmail log should show `relay=smtp.gmail.com` and a successful `dsn=2.0.0`.

## Choosing between the two

| Situation | Recommended transport |
|---|---|
| Only rgbdns needs outbound SMTP submission | `msmtp` |
| Existing services already use `/usr/sbin/sendmail` | Native Sendmail |
| You want the smallest configuration and no MTA daemon | `msmtp` |
| You need a local mail queue and conventional MTA behavior | Native Sendmail |
| You want the shortest troubleshooting path | `msmtp` |
| You already operate Sendmail and understand its macros/SASL | Native Sendmail |

Do not configure both transports for the report at once. The active transport
is determined solely by `REPORT_SENDMAIL`:

```sh
sudo grep '^REPORT_SENDMAIL=' /etc/rgbdns/query-report.env
sudo readlink -f "$(sudo awk -F= '/^REPORT_SENDMAIL=/{print $2}' /etc/rgbdns/query-report.env)"
```

`/usr/bin/msmtp` means the report uses msmtp. `/usr/sbin/sendmail` resolving
to the Sendmail executable and logs containing `sm-mta` mean it uses native
Sendmail.

## Delivery troubleshooting checklist

Run these commands on the primary:

```sh
sudo systemctl status rgbdns-query-report.service --no-pager
sudo journalctl -u rgbdns-query-report.service -n 50 --no-pager
sudo journalctl -u sendmail --since "15 minutes ago" --no-pager
sudo mailq
```

Interpret the results:

- `status=0/SUCCESS` means the report transport accepted the message. It does
  not by itself prove Gmail delivered it.
- `dsn=2.0.0` and `stat=Sent` for the Resend relay indicate successful handoff
  to Resend.
- `535 Authentication credentials invalid` means the Resend SMTP credentials
  are wrong or the auth map was not rebuilt.
- `550 ... domain is not verified` means the visible sender domain is not
  verified in Resend.
- `550 5.7.26 ... unauthenticated` together with a Gmail MX relay means the
  host is still delivering directly instead of using Resend.
- An empty queue after a permanent failure is expected; Sendmail rejected the
  message rather than retaining it for retry.

Check the recipient's Spam folder only after the relay log shows successful
handoff to Resend.
