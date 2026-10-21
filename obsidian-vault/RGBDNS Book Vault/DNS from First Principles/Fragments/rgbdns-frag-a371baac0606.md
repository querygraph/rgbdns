---
type: "code-fragment"
fragment_id: "rgbdns-frag-a371baac0606"
source_path: "RESEND_SENDMAIL.md"
code_note: "DNS from First Principles/Code/RESEND_SENDMAIL.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Resend mail delivery for rgbdns reports"
kind: "heading"
start_line: 1
end_line: 14
---

# Resend mail delivery for rgbdns reports

- Fragment ID: `rgbdns-frag-a371baac0606`
- Source file: [[DNS from First Principles/Code/RESEND_SENDMAIL.md.source|RESEND_SENDMAIL.md]]
- Lines: 1-14
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-a371baac0606", "codeNote": "DNS from First Principles/Code/RESEND_SENDMAIL.md.source", "sourcePath": "RESEND_SENDMAIL.md", "startLine": 1, "endLine": 14}
```

## Excerpt

<span id="rgbdns-frag-a371baac0606" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a371baac0606: heading Resend mail delivery for rgbdns reports

````markdown
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
````
