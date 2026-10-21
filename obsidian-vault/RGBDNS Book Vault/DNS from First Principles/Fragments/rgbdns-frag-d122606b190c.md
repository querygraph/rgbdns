---
type: "code-fragment"
fragment_id: "rgbdns-frag-d122606b190c"
source_path: "RESEND_SENDMAIL.md"
code_note: "DNS from First Principles/Code/RESEND_SENDMAIL.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "etc rgbdns query-report.env"
kind: "heading"
start_line: 15
end_line: 23
---

# etc rgbdns query-report.env

- Fragment ID: `rgbdns-frag-d122606b190c`
- Source file: [[DNS from First Principles/Code/RESEND_SENDMAIL.md.source|RESEND_SENDMAIL.md]]
- Lines: 15-23
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-d122606b190c", "codeNote": "DNS from First Principles/Code/RESEND_SENDMAIL.md.source", "sourcePath": "RESEND_SENDMAIL.md", "startLine": 15, "endLine": 23}
```

## Excerpt

<span id="rgbdns-frag-d122606b190c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d122606b190c: heading etc rgbdns query-report.env

````markdown
# /etc/rgbdns/query-report.env
REPORT_TO=deliverable@gmail.com
REPORT_FROM=rgbdns@hurz.net
REPORT_SENDMAIL=/usr/sbin/sendmail
```

`REPORT_FROM` must use a domain verified in Resend. The hostname of the DNS
server, such as `a.ns.cron.sh`, does not need to be the mail sender domain.

````
