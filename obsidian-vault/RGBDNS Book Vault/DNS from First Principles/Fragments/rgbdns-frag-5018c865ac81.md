---
type: "code-fragment"
fragment_id: "rgbdns-frag-5018c865ac81"
source_path: "RESEND_SENDMAIL.md"
code_note: "DNS from First Principles/Code/RESEND_SENDMAIL.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Configure the rgbdns report"
kind: "heading"
start_line: 58
end_line: 90
---

# Configure the rgbdns report

- Fragment ID: `rgbdns-frag-5018c865ac81`
- Source file: [[DNS from First Principles/Code/RESEND_SENDMAIL.md.source|RESEND_SENDMAIL.md]]
- Lines: 58-90
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-5018c865ac81", "codeNote": "DNS from First Principles/Code/RESEND_SENDMAIL.md.source", "sourcePath": "RESEND_SENDMAIL.md", "startLine": 58, "endLine": 90}
```

## Excerpt

<span id="rgbdns-frag-5018c865ac81" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5018c865ac81: heading Configure the rgbdns report

````markdown
## Configure the rgbdns report

On the primary only, edit the report configuration:

```sh
sudo vi /etc/rgbdns/query-report.env
```

Use a verified sender address:

```text
REPORT_TO=deliverable@gmail.com
REPORT_FROM=rgbdns@hurz.net
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

````
