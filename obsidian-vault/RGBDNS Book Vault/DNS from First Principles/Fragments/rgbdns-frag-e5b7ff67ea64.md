---
type: "code-fragment"
fragment_id: "rgbdns-frag-e5b7ff67ea64"
source_path: "RESEND_SENDMAIL.md"
code_note: "DNS from First Principles/Code/RESEND_SENDMAIL.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Delivery troubleshooting checklist"
kind: "heading"
start_line: 707
end_line: 738
---

# Delivery troubleshooting checklist

- Fragment ID: `rgbdns-frag-e5b7ff67ea64`
- Source file: [[DNS from First Principles/Code/RESEND_SENDMAIL.md.source|RESEND_SENDMAIL.md]]
- Lines: 707-738
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-e5b7ff67ea64", "codeNote": "DNS from First Principles/Code/RESEND_SENDMAIL.md.source", "sourcePath": "RESEND_SENDMAIL.md", "startLine": 707, "endLine": 738}
```

## Excerpt

<span id="rgbdns-frag-e5b7ff67ea64" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e5b7ff67ea64: heading Delivery troubleshooting checklist

````markdown
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
- `550 5.7.26 ... unauthenticated` together with a `gmail-smtp-in` relay means
  direct MX delivery reached Gmail but failed sender authentication. Use the
  authenticated Gmail smarthost or configure full direct-mail authentication.
- An empty queue after a permanent failure is expected; Sendmail rejected the
  message rather than retaining it for retry.

Check the recipient's Spam folder only after the relay log shows successful
handoff to Resend or Gmail. Avoid `sendmail -v` when an auth map is configured:
verbose SMTP output can expose the Base64-encoded SMTP credential. If that
happens, revoke the exposed Resend key or Gmail App Password and replace it in
`/etc/mail/authinfo` before testing again.
````
